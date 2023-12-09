from regex import search, REVERSE


def solution():
    DIGITS = r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9"

    runningSum = 0
    lines = open("data/day1.txt").read().splitlines()
    for line in lines:
        if (left := search(DIGITS, line).group()) and (
            right := search(DIGITS, line, REVERSE).group()
        ):
            left = left if left.isdigit() else left.toDigit()
            right = right if right.isdigit() else right.toDigit()
            runningSum += int(left + right)

    print(runningSum)


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
