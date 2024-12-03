from typing import Callable
from operator import lt, gt
from itertools import pairwise


def monotony(levels: tuple[int, ...], op: Callable[[int, int], bool]) -> bool:
    for i in range(1, len(levels)):
        if not op(levels[i], levels[i - 1]):
            return False
    return True


def distance_in_range(levels: tuple[int, ...]) -> bool:
    for a, b in pairwise(levels):
        if (d := abs(a - b)) < 1 or d > 3:
            return False
    return True


def is_safe(levels: tuple[int, ...]) -> bool:
    return distance_in_range(levels) and (monotony(levels, lt) or monotony(levels, gt))


def can_dampen(levels: tuple[int, ...]) -> bool:
    for i in range(len(levels)):
        if is_safe(levels[:i] + levels[i + 1 :]):
            return True
    return False


if __name__ == "__main__":
    safe_reports, dampened = 0, 0
    with open("input") as input:
        for line in input:
            levels = tuple(map(int, line.split()))
            if is_safe(levels):
                safe_reports += 1
            else:
                dampened += can_dampen(levels)
    print(safe_reports, safe_reports + dampened, sep="\n")
