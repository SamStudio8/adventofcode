#!/usr/bin/env python
import sys
from ast import literal_eval
import functools

def cmp(left, right):
    if isinstance(left, int) and isinstance(right, int):
        return (left > right) - (left < right)
    elif isinstance(left, int):
        return cmp([left], right)
    elif isinstance(right, int):
        return cmp(left, [right])
    else:
        # map(f, zip(...)) can just be written map(f, z1, z2)
        for res in map(cmp, left, right):
            if res:
                return res
        # reached end of left without making a decision
        #   if left longer, its larger, therefore unsorted
        return cmp(len(left), len(right))
    return -1


s = 0
lines = []
pairs = sys.stdin.read().split("\n\n")
for (i, pair) in enumerate(pairs):
    i += 1
    left, right = [literal_eval(s) for s in pair.strip().split('\n')]
    lines.append(left)
    lines.append(right)
    cmp_score = cmp(left, right)
    print("Pair", i, cmp_score)
    if cmp_score == -1:
        s += i
print(f"Sorted pairs: {s}")

dividers = [
    [[2]],
    [[6]],
]
lines.extend(dividers)
slines = sorted(lines, key=functools.cmp_to_key(cmp))
part2 = 1
for d in dividers:
    part2 *= slines.index(d)+1
print(f"Decoder key: {part2}")


