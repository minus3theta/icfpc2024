#!/usr/bin/env python
import click

head = "B$ B$ Lf B$ Lx B$ vf B$ vx vx Lx B$ vf B$ vx vx Lf Lx ? B< vx I% S B$ L$ B. B$ B$ Ls Ln BT I\" BD vn vs SOL>F B% vx I% B$ vf v$ B/ vx I% I"

def base94(x: int):
    if x == 0:
        return chr(33)
    s = ""
    while x > 0:
        x, r = divmod(x, 94)
        s = chr(r + 33) + s
    return s

code = { 'U': 0, 'R': 1, 'D': 2, 'L': 3 }

@click.command()
@click.argument("src", type=click.Path(exists=True, file_okay=True), default="data/lambdaman/lambdaman1.koudai.dfs.out")
def main(src):
    acc = 1  # 1 for sentinel
    with open(src, "r") as f:
        content = reversed(f.read().rstrip())
        for ch in content:
            acc = acc * 4 + code[ch]

    print(head + base94(acc))

if __name__ == "__main__":
    main()
