def main():
    with open("data/day.txt") as file:
        data = makeList(file)
        result = elfCarryingMostCalories(data)
        # For the part two
        secondPlace = elfCarryingMostCalories(data)
        thirdPlace = elfCarryingMostCalories(data)
        print(result + secondPlace + thirdPlace)

def makeList(f):
    current = []
    numbers = []

    for line in f:
        lineConv = list(line)
        if len(lineConv) == 1 and lineConv[0] == '\n':
            numbers.append(current)
            current = []
        else:
            lineConv.pop(len(lineConv) - 1)
            current.append(int(''.join(lineConv)))

    return numbers

def elfCarryingMostCalories(cal):
    largest = sum(cal[0])
    tmpSum = 0
    global largestIndex
    largestIndex = 0

    for i in range(len(cal)):
        for j in range(len(cal[i])):
            tmpSum += cal[i][j]

        if tmpSum > largest:
            largest = tmpSum
            largestIndex = i

        tmpSum = 0

    cal.pop(largestIndex)

    return largest

main()
