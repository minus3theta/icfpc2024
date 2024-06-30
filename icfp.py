#!/usr/bin/env python
import sys
import os
import json
import re
import subprocess
import tempfile


def parse_base94(s: str):
    # cps = [ord(c)-33 for c in s]
    # print(s, cps)
    x = 0
    for c in s:
        cp = ord(c) - 33
        assert 0 <= cp < 94
        x = 94 * x + cp
    return x
    # assert c in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_`{|}~"


def base94(x: int):
    if x == 0:
        return chr(33)
    s = ""
    while x > 0:
        x, r = divmod(x, 94)
        s = chr(r + 33) + s
    return s
    # assert c in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+,-./:;<=>?@[]^_`{|}~"


table = str.maketrans(
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
)
rev_table = str.maketrans(
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n",
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
)

def decode_string(s):
    t = s.translate(table)
    # print(f":: {s} -> {t}")
    return t

def encode_string(s):
    t = s.translate(rev_table)
    # print(f":: {s} -> {t}")
    return t


class Token:
    def __init__(self, token_str: str):
        self.token_str = token_str
        self.evaled = self
        # self.indicator = token_str[0]
        # self.body = token_str[1:]

    def __str__(self):
        return self.token_str

    def __repr__(self):
        # return self.token_str
        return f"<{self.token_str[0]} {self.token_str[1:]}>"

    def __hash__(self):
        return hash(self.token_str)

    def eval(self, env={}):
        return self

    @classmethod
    def from_string(cls, token_str: str):
        if len(token_str) == 0:
            return Undefined_

        indicator = token_str[0]
        body = token_str[1:]

        if indicator == "T":
            assert body == ""
            token = Boolean(True)
        elif indicator == "F":
            assert body == ""
            token = Boolean(False)
        elif indicator == "I":
            token = Integer(parse_base94(body))
        elif indicator == "S":
            token = String(body) # decode_string(body))
        elif indicator == "U":
            assert len(body) == 1
            token = UnaryOperator(body)
        elif indicator == "B":
            assert len(body) == 1
            token = BinaryOperator(body)
        elif indicator == "?":
            assert body == ""
            token = If()
        elif indicator == "L":
            token = Lambda(parse_base94(body))
        elif indicator == "v":
            token = Variable(parse_base94(body))
        else:
            return Undefined_

        token.token_str = token_str
        return token


class Undefined(Token):
    def __init__(self):
        self.token_str = ""
        # self.evaled = self

    def __str__(self):
        return "<undef>"

    def __repr__(self):
        return "<undef>"

    def eval(self):
        return self


Undefined_ = Undefined()


class Boolean(Token):
    def __init__(self, val: bool):
        self.val = val
        self.token_str = "T" if val else "F"
        self.evaled = self

    def __repr__(self):
        return "true" if self.val else "false"

    def __eq__(self, other):
        return self.val == other.val


class Integer(Token):
    def __init__(self, val: int):
        self.token_str = "I"
        self.val = val
        self.evaled = self

    def __str__(self):
        return str(self.val)

    def __repr__(self):
        return f"{self.val}"

    def __eq__(self, other):
        return self.val == other.val


class Lambda(Token):
    def __init__(self, var: int):
        self.token_str = "L"
        self.var = var
        self.proc = None
        self.lmd = None
        self.evaled = self
        self.env = {var: Undefined_}

    def __str__(self):
        return f"λ{self.var}={repr(self.proc)}"

    def __repr__(self):
        if self.proc is None:
            return f"λ{self.var}"
        else:
            return f"λ{self.var}={repr(self.proc)}"

    # def eval(self, env={}, arg=None):
    #     return self


class Variable(Token):
    def __init__(self, var: int):
        self.token_str = "v"
        self.var = var
        self.evaled = None

    def __str__(self):
        return str(self.var)

    def __repr__(self):
        return f"var{self.var}"

    def eval(self, env={}):
        if self.evaled is not None:
            return self.evaled
        # assert self.var in env
        res = env.get(self.var, Undefined_)
        # print(f"eval var {self.var} = {res}")
        self.evaled = res.eval()
        return self.evaled


class String(Token):
    def __init__(self, val: str):
        self.token_str = "S"
        self.val = val
        self.evaled = self

    def __str__(self):
        return self.val

    def __repr__(self):
        return f'"{decode_string(self.val)}"'

    def __eq__(self, other):
        return self.val == other.val

    @classmethod
    def from_human_string(cls, s: str):
        return cls(encode_string(s))


class UnaryOperator(Token):
    def __init__(self, op: str):
        self.token_str = None
        assert op in "-!#$"
        self.op = op
        self.arg = None
        self.evaled = None

    def __repr__(self):
        if self.arg is None:
            return f"({self.op} _)"
        else:
            return f"({self.op} {repr(self.arg)})"

    def eval(self, env={}):
        if self.evaled is not None:
            return self.evaled

        x = self.arg
        if self.op == "-":
            assert x.__class__ == Integer
            self.evaled = Integer(-x.val)
        elif self.op == "!":
            assert x.__class__ == Boolean
            self.evaled = Boolean(not x.val)
        elif self.op == "#":
            assert x.__class__ == String
            self.evaled = Integer(parse_base94(x.val))
        elif self.op == "$":
            assert x.__class__ == Integer
            self.evaled = String(base94(x.val))
        return self.evaled


class BinaryOperator(Token):
    def __init__(self, op: str):
        self.token_str = None
        # assert type in "-!#$"
        self.op = op
        self.arg1 = None
        self.arg2 = None
        self.evaled = None

    def __repr__(self):
        if self.op == "$":
            if self.arg1 is None:
                return "apply"
            else:
                return f"(apply {repr(self.arg1)} {repr(self.arg2)})"
        else:
            if self.arg1 is None:
                return f"({self.op} _ _)"
            else:
                return f"({self.op} {repr(self.arg1)} {repr(self.arg2)})"

    def eval(self, env={}):
        if self.evaled is not None:
            return self.evaled

        if self.op == "$":  # "B$"
            x = self.arg1
            while x.__class__ != Lambda:
                x = x.eval(env=env)

            # if x.__class__ == Variable:  # は？
            #     return x.eval(env=env)

            y = self.arg2.eval(env=env)
            # print(f":: B$, x.class = {x.__class__}")

            assert x.__class__ == Lambda

            newenv = env.copy()
            newenv.update(x.env)

            if x.proc.__class__ == Lambda:
                # 答えはLambdaになる
                newenv.update(x.env)
                newenv[x.var] = y
                # print(f"B$ env (L) = {newenv}/{env}, {repr(x.proc)}, after adding #{x.var} = {y}")
                next = Lambda(x.proc.var)
                next.proc = x.proc.proc
                next.lmd = x.proc.lmd
                next.env = newenv
                self.evaled = next
            else:
                newenv[x.var] = y
                # print(f"B$ env = {newenv}/{env}, {repr(x.proc)}, after adding #{x.var} = {y}")
                # もう評価する
                self.evaled = x.proc.eval(env=newenv)
            return self.evaled

        x = self.arg1.eval(env=env)
        y = self.arg2.eval(env=env)
        # print(f":: {self.op} x={x}, y={y}")
        if self.op == "+":
            assert x.__class__ == Integer and y.__class__ == Integer
            self.evaled = Integer(x.val + y.val)
        elif self.op == "-":
            assert x.__class__ == Integer and y.__class__ == Integer
            self.evaled = Integer(x.val - y.val)
        elif self.op == "*":
            assert x.__class__ == Integer and y.__class__ == Integer
            self.evaled = Integer(x.val * y.val)
        elif self.op == "/":
            assert x.__class__ == Integer and y.__class__ == Integer
            sign = 1 if x.val >= 0 else -1
            sign *= 1 if y.val >= 0 else -1
            self.evaled = Integer(sign * (abs(x.val) // abs(y.val)))   # TRUNCATE TOWARDS ZERO
        elif self.op == "%":
            assert x.__class__ == Integer and y.__class__ == Integer
            sign = 1 if x.val >= 0 else -1
            sign *= 1 if y.val >= 0 else -1
            self.evaled = Integer(sign * (abs(x.val) % abs(y.val)))
        elif self.op == "<":
            assert x.__class__ == Integer and y.__class__ == Integer
            self.evaled = Boolean(x.val < y.val)
        elif self.op == ">":
            assert x.__class__ == Integer and y.__class__ == Integer
            self.evaled = Boolean(x.val > y.val)
        elif self.op == "=":
            self.evaled = Boolean(x.val == y.val)
        elif self.op == "|":
            assert x.__class__ == Boolean and y.__class__ == Boolean
            self.evaled = Boolean(x.val or y.val)
        elif self.op == "&":
            assert x.__class__ == Boolean and y.__class__ == Boolean
            self.evaled = Boolean(x.val and y.val)
        elif self.op == ".":
            assert x.__class__ == String and y.__class__ == String
            self.evaled = String(x.val + y.val)
        elif self.op == "T":
            assert x.__class__ == Integer and y.__class__ == String
            self.evaled = String(y.val[:x.val])
        elif self.op == "D":
            assert x.__class__ == Integer and y.__class__ == String
            self.evaled = String(y.val[x.val:])
        elif self.op == "$":  # "B$""
            assert x.__class__ == Lambda # or x.__class__ == function
            newenv = env.copy()
            # env[x.var] = y
            # print(f"B$ env = {env}, after adding #{x.var} = {y}")
            # self.evaled = x.lmd(env)
            newenv[x.var] = y
            print(f"B$ env = {newenv}/{env}, after adding #{x.var} = {y}")
            self.evaled = x.lmd(newenv)
            if self.evaled.__class__ == Lambda:
                self.evaled.env = newenv
        else:
            raise f"Unkonwn operator {self.op}"

        return self.evaled


class If(Token):
    def __init__(self):
        self.token_str = None
        self.arg1 = None
        self.arg2 = None
        self.arg3 = None
        self.evaled = None

    def __repr__(self):
        if self.arg1 is None:
            return f"(if _ _ _)"
        else:
            return f"(if {repr(self.arg1)} {repr(self.arg2)} {repr(self.arg3)})"

    def eval(self, env={}):
        if self.evaled is not None:
            return self.evaled

        if self.arg1.eval(env=env).val == True:
            self.evaled = self.arg2.eval(env=env)
        else:
            self.evaled = self.arg3.eval(env=env)

        return self.evaled


def make_ast(tokens):
    def eat_one(from_ofs) -> (Token, int):
        o = from_ofs
        while o < len(tokens):
            item = tokens[o]
            o += 1
            if item.__class__ == UnaryOperator:
                item.arg, o = eat_one(o)
            elif item.__class__ == BinaryOperator:
                item.arg1, o = eat_one(o)
                item.arg2, o = eat_one(o)
            elif item.__class__ == If:
                item.arg1, o = eat_one(o)
                item.arg2, o = eat_one(o)
                item.arg3, o = eat_one(o)
            elif item.__class__ == Lambda:
                item.proc, o = eat_one(o)
                item.lmd = item.proc.eval
            else:
                pass
            return item, o
        return Undefined_, len(tokens)

    stack = []
    o = 0
    while o < len(tokens):
        item, o = eat_one(o)
        stack.append(item)

    return stack


def make_z_combinator(f, x, y):
    #"B$ L~f B$ L~x B$ v~f L~y B$ B$ v~x v~x v~y L~x B$ v~f L~y B$ B$ v~x v~x v~y")
    return f"B$ L{f} B$ L{x} B$ v{f} L{y} B$ B$ v{x} v{x} v{y} L{x} B$ v{f} L{y} B$ B$ v{x} v{x} v{y}"

def replace_y_combinator(s):
    return re.sub(r"B\$ L(.) B\$ L(.) B\$ v(.) B\$ v(.) v(.) L(.) B\$ v(.) B\$ v(.) v(.)", make_z_combinator('"', '#', '%'), s)
    # if mo is not None:
    #     print(":detect Y combinator")
    # return s

def parse_icfp(s):
    t = replace_y_combinator(s)
    if t != s:
        print(":Y:", s, " --> ", t)
    tokens = [Token.from_string(token_str) for token_str in re.split("[ \t\r\n]+", s)]
    print("  tokens:", ' '.join([repr(token) for token in tokens]))
    return make_ast(tokens)



def my_test(icfp_expr, expected):
    print("source:", icfp_expr)
    parsed = parse_icfp(icfp_expr)
    print("  parsed:", repr(parsed[0]))
    result = parsed[0].eval()
    print("  result:", repr(result))
    print("  expected:", repr(expected))
    assert result == expected


if __name__ == '__main__':
    print(sys.getrecursionlimit())
    sys.setrecursionlimit(10000000)
    print(sys.getrecursionlimit())
    my_test("S'%4}).$%8", String.from_human_string("get index"))

    # Booleans
    my_test("T", Boolean(True))
    my_test("F", Boolean(False))

    # Integers
    my_test("I/6", Integer(1337))

    # Strings
    my_test("SB%,,/}Q/2,$_", String.from_human_string("Hello World!"))

    # Unaryu opeartors
    my_test("U- I$", Integer(-3))
    my_test("U! T", Boolean(False))
    my_test("U# S4%34", Integer(15818151))
    my_test("U$ I4%34", String.from_human_string("test"))

    # Binary operators
    my_test("B+ I# I$", Integer(5))
    my_test("B- I$ I#", Integer(1))
    my_test("B* I$ I#", Integer(6))
    my_test("B/ U- I( I#", Integer(-3))
    my_test("B% U- I( I#", Integer(-1))
    my_test("B< I$ I#", Boolean(False))
    my_test("B> I$ I#", Boolean(True))
    my_test("B= I$ I#", Boolean(False))
    my_test("B| T F", Boolean(True))
    my_test("B& T F", Boolean(False))
    my_test("B. S4% S34", String.from_human_string("test"))
    my_test("BT I$ S4%34", String.from_human_string("tes"))
    my_test("BD I$ S4%34", String.from_human_string("t"))

    # If
    my_test("? B> I# I$ S9%3 S./", String.from_human_string("no"))

    # Lambda
    my_test('B$ B$ L# L$ v# B. SB%,,/ S}Q/2,$_ IK', String.from_human_string("Hello World!"))
    my_test('B$ L# B$ L" B+ v" v" B* I$ I# v8', Integer(12))
    my_test('B$ B$ L" B$ L# B$ v" B$ v# v# L# B$ v" B$ v# v# L" L# ? B= v# I! I" B$ L$ B+ B$ v" v$ B$ v" v$ B- v# I" I%', Integer(16))
