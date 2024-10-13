def firstPart():
    file = open('data/day2.txt')
    myScore = 0

    # A: rock
    # B: paper
    # C: scissors

    # X: rock
    # Y: paper
    # Z: scissors

    for line in file:
        if line[0] == 'A' and line[2] == 'Y' or \
             line[0] == 'B' and line[2] == 'Z' or \
             line[0] == 'C' and line[2] == 'X':
            myScore += 6
        elif line[0] == 'A' and line[2] == 'X' or \
                 line[0] == 'B' and line[2] == 'Y' or \
                 line[0] == 'C' and line[2] == 'Z':
            myScore += 3

        if line[2] == 'X' : myScore += 1
        if line[2] == 'Y' : myScore += 2
        if line[2] == 'Z' : myScore += 3

    print(myScore)

def secondPart():
    file = open('day2.txt')
    myScore = 0

    # A: rock
    # B: paper
    # C: scissors

    # X: lose
    # Y: draw
    # Z: win


    for line in file:
        op = line[0]
        out = line[2]

        if out == 'X':
            if op == 'A':
                myScore += 3
            elif op == 'B':
                myScore += 1
            elif op == 'C':
                myScore += 2

        if out == 'Y':
            if op == 'A':
                myScore += 1 + 3
            elif op == 'B':
                myScore += 2 + 3
            elif op == 'C':
                myScore += 3 + 3

        if out == 'Z':
            if op == 'A':
                myScore += 2 + 6
            elif op == 'B':
                myScore += 3 + 6
            elif op == 'C':
                myScore += 1 + 6

    print(myScore)

secondPart()
