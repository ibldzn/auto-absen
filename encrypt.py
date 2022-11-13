#!/usr/bin/env python3

import sys


def encrypt(passwd: str) -> str:
    return ",".join([str(ord(c) ^ ord("A")) for c in passwd])


def main():
    args = sys.argv
    if len(args) != 2:
        print(f"Usage: {args[0]} <password-to-encrypt>", file=sys.stderr)
        return
    print(encrypt(args[1]))


if __name__ == "__main__":
    main()
