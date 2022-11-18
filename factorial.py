#! /usr/bin/env python3
import sys

args = sys.argv

def factorial(x):
    if x == 1:
        return 1
    else:
        return x * factorial(x - 1)


print(factorial(int(args[1])))
