from functools import cache
from operator import and_, or_, lshift, rshift

OPS = {"AND": and_, "OR": or_, "LSHIFT": lshift, "RSHIFT": rshift}


@cache
def signal(wire: str):
    if wire.isdigit():
        return int(wire)

    match wires[wire]:
        case [a]:
            return signal(a)
        case ["NOT", a]:
            return ~signal(a)
        case [a, op, b]:
            return OPS[op](signal(a), signal(b))


with open("input") as input:
    wires = {}
    for line in input.readlines():
        *expr, _, dst = line.split()
        wires[dst] = expr

    print(signal("a"))

    wires["b"] = [str(signal("a"))]
    signal.cache_clear()
    print(signal("a"))
