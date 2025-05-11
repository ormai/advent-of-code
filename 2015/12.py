import re
import json


def sum_numbers(tree) -> int:
    if isinstance(tree, int):
        return tree
    if isinstance(tree, dict):
        if "red" not in tree.values():
            return sum(sum_numbers(value) for value in tree.values())
    if isinstance(tree, list):
        return sum(sum_numbers(value) for value in tree)
    return 0


def main():
    input = open("input").readline()
    print(sum(int(n) for n in re.findall(r"-?\d+", input)))  # part one
    print(sum_numbers(json.loads(input)))  # part two


if __name__ == "__main__":
    main()
