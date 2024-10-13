import re

with open("input") as input:
    lines = input.readlines()

    # part one
    print(sum(int((d := re.findall(r"\d", line))[0] + d[-1]) for line in lines))

    # part two
    D = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

    def to_digit(s):
        return str(D.index(s) + 1) if s in D else s

    # A few lines have overlapping words, e.g. 'eightwo',
    # that's what the positive lookahead '?=' is for
    N = re.compile(rf"(?=({'|'.join(D)}|\d))")
    print(
        sum(
            int((d := [*map(to_digit, re.findall(N, line))])[0] + d[-1])
            for line in lines
        )
    )
