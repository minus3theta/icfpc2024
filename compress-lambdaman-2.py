#!/usr/bin/env python
import click

head = "B$ B$ Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx Lf Lx ? B= vx I! S B$ L$ B. B$ Lc B$ B$ Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx Lf Lx ? B= vx I! S B. vc B$ vf B- vx I\" B% vx I\"! U$ B% B/ vx I\"! I\"! B$ vf v$ B/ vx I\"!! I"

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

def rle(content):
    result = []
    last = None
    cnt = 0
    for ch in content:
        if ch not in "OL>F": break
        if ch == last:
            cnt += 1
            if cnt == 93:
                result.append((last, cnt))
                last = None
                cnt = 0
        else:
            if last is not None:
                result.append((last, cnt))
            last = ch
            cnt = 1
    if last is not None:
        result.append((last, cnt))
    return result

# code = { 'U': 0, 'R': 1, 'D': 2, 'L': 3 }
# code = { 'U': 'O', 'R': 'L', 'D': '>', 'L': 'F' }

@click.command()
@click.argument("src", type=click.Path(exists=True, file_okay=True), default="data/lambdaman/lambdaman1.koudai.dfs.out")
def main(src):
    result = ""
    with open(src, "r") as f:
        content = f.read().rstrip()
        encoded = encode_string(content)
        for ch, count in reversed(rle(encoded)):
            result += ch + chr(count + 33)

    print(head + result)

if __name__ == "__main__":
    main()
