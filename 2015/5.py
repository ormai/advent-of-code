import re


def part_one_rules(s):
    return (
        len(re.findall(r"[aeiou]", s)) >= 3
        and re.search(r"(\w)\1", s)
        and not re.search(r"ab|cd|pq|xy", s)
    )


def part_two_rules(s):
    return re.search(r"(\w{2}).*(\1)", s) and re.search(r"(\w)[^\1](\1)", s)


with open("input") as input:
    strings = input.readlines()
    print(len(list(filter(part_one_rules, strings))))
    print(len(list(filter(part_two_rules, strings))))
