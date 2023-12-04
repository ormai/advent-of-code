import regex


def solution():
    with open("data/day1.txt") as file:
        DIGIT = r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9"
        sum = 0
        for line in file:
            value = ""

            l = regex.search(DIGIT, line)
            if l:
                value += l.group() if l.group().isdigit() else toDigit(l.group())

            r = regex.search(DIGIT, line, regex.REVERSE)
            if r:
                value += r.group() if r.group().isdigit() else toDigit(r.group())

            sum += int(value)
        print(sum)


def toDigit(word):
    match word:
        case "one":
            return "1"
        case "two":
            return "2"
        case "three":
            return "3"
        case "four":
            return "4"
        case "five":
            return "5"
        case "six":
            return "6"
        case "seven":
            return "7"
        case "eight":
            return "8"
        case "nine":
            return "9"


solution()
