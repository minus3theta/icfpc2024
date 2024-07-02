#!/usr/bin/env python
import click

def base94(x: int):
    if x == 0:
        return chr(33)
    s = ""
    while x > 0:
        x, r = divmod(x, 94)
        s = chr(r + 33) + s
    return s

table = str.maketrans(
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n"
)
rev_table = str.maketrans(
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n",
    "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
)

def decode_string(s: str):
    t = s.translate(table)
    # print(f":: {s} -> {t}")
    return t

def encode_string(s: str):
    t = s.translate(rev_table)
    # print(f":: {s} -> {t}")
    return t

def main():
    s = input()
    print(decode_string(s))

if __name__ == '__main__':
    main()
