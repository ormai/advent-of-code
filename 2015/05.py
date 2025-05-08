import re


def part_one_rules(s: str) -> bool:
    return (
        len(re.findall(r"[aeiou]", s)) >= 3
        and re.search(r"(\w)\1", s) is not None
        and not re.search(r"ab|cd|pq|xy", s) is not None
    )


def part_two_rules(s: str) -> bool:
    return (
        re.search(r"(\w{2}).*(\1)", s) is not None
        and re.search(r"(\w)[^\1](\1)", s) is not None
    )


with open("input") as input:
    strings = input.readlines()
    print(len(list(filter(part_one_rules, strings))))
    print(len(list(filter(part_two_rules, strings))))
