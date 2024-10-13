import re

with open("input") as input:
    lines = input.readlines()

    # part one
    print(sum(int((d := re.findall(r"\d", line))[0] + d[-1]) for line in lines))

    # part two
    D = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    def to_digit(s):
        return str(D.index(s) + 1) if s in D else s

    total = 0
    for line in lines:
        # A few lines have overlapping words, e.g. 'eightwo',
        # that's what the positive lookahead '?=' is for
        d = [*map(to_digit, re.findall(rf"(?=({r'|'.join(D)}|\d))", line))]
        total += int(d[0] + d[-1])
    print(total)
