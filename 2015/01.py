with open("input") as input:
    floor = position = 0
    line = input.readline()
    for i in range(len(line)):
        if line[i] == "(":
            floor += 1
        else:
            floor -= 1
        if position == 0 and floor == -1:
            position = i
    print(floor)  # part one
    print(position + 1)  # part two
