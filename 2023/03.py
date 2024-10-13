import re


def partOne():
    with open("data/day3.txt") as file:
        schematic = [line for line in file]

        SYMBOLS = r"\*|\+|\$|#|/|-|@|%|&|="
        sum = 0

        for i in range(len(schematic)):
            start = end = -1

            print(schematic[i])
            for j in range(len(schematic[0])):
                if schematic[i][j].isdigit():
                    if start == -1:
                        start = j

                if (not schematic[i][j].isdigit() and start != -1) or j + 1 == len(
                    schematic[0]
                ):
                    substringStart = start
                    substringEnd = end = j

                    if substringStart - 1 >= 0:
                        substringStart -= 1
                    if substringEnd + 1 <= len(schematic[i]):
                        substringEnd += 1

                    if (
                        (
                            i - 1 >= 0
                            and re.search(
                                SYMBOLS, schematic[i -
                                                   1][substringStart:substringEnd]
                            )
                        )
                        or (
                            i + 1 < len(schematic)
                            and re.search(
                                SYMBOLS, schematic[i +
                                                   1][substringStart:substringEnd]
                            )
                        )
                        or re.search(SYMBOLS, schematic[i][substringStart:substringEnd])
                    ):
                        sum += int(schematic[i][start:end])

                    start = -1
        print(sum)


def partTwo():
    with open("day3.txt") as file:
        schematic = [line for line in file]

        sum = 0

        for i in range(len(schematic)):
            print(schematic[i])
            for j in range(len(schematic[0])):
                if schematic[i][j] == "*":
                    sum += gearRatio(schematic, i, j)

        print(sum)


def gearRatio(mat, i, j):
    leftBarrier = j - 3
    rightBarrier = j + 3

    numbers = []

    if j - 3 < 0:
        leftBarrier = 0
    if j + 3 >= len(mat[i]):
        rightBarrier = len(mat[i])

    if i - 1 >= 0:
        begin = end = 0
        if j - 1 >= 0:
            if mat[i - 1][j - 1].isdigit():
                begin = leftBarrier
            else:
                begin = j

        if j + 1 < len(mat[i]):
            if mat[i - 1][j + 1].isdigit():
                end = rightBarrier
            else:
                end = j

        n = ""
        for k in range(begin, end + 1):
            if mat[i - 1][k].isdigit():
                n += mat[i - 1][k]

            if (not mat[i - 1][k].isdigit() or k == end) and n != "":
                if len(n) == 1 and (k - 1 > j + 1 or k - 1 < j - 1):
                    n = ""
                    continue
                numbers.append(n)
                n = ""

    if i + 1 < len(mat):
        begin = end = 0
        if j - 1 >= 0:
            if mat[i + 1][j - 1].isdigit():
                begin = leftBarrier
            else:
                begin = j
        if j + 1 < len(mat[i]):
            if mat[i + 1][j + 1].isdigit():
                end = rightBarrier
            else:
                end = j

        n = ""
        for k in range(begin, end + 1):
            if mat[i + 1][k].isdigit():
                n += mat[i + 1][k]
            if (not mat[i + 1][k].isdigit() or k == end) and n != "":
                if len(n) == 1 and (k - 1 > j + 1 or k - 1 < j - 1):
                    n = ""
                    continue
                numbers.append(n)
                n = ""

    if j - 1 >= 0 and mat[i][j - 1].isdigit():
        numbers.append(mat[i][leftBarrier:j].lstrip(".*$#@/=+-&"))

    if j + 1 <= len(mat[i]) and mat[i][j + 1].isdigit():
        numbers.append(mat[i][j + 1: rightBarrier + 1].rstrip(".*$#@/=+-&"))

    if len(numbers) < 2:
        return 0

    print(numbers)

    return int(numbers[0]) * int(numbers[1])


partTwo()
