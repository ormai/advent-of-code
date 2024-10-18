SHAPES = {"X": 0, "Y": 1, "Z": 2, "A": 0, "B": 1, "C": 2}
with open("input") as tournament:
    score_p1 = score_p2 = 0
    for round in tournament:
        opp, me = round.split()
        score_p1 += SHAPES[me] + 1
        if SHAPES[opp] == SHAPES[me]:
            score_p1 += 3
        elif (SHAPES[opp] + 1) % 3 == SHAPES[me]:
            score_p1 += 6

        # part two
        if me == "X":
            score_p2 += (SHAPES[opp] - 1) % 3
        elif me == "Y":
            score_p2 += SHAPES[opp] + 3
        elif me == "Z":
            score_p2 += (SHAPES[opp] + 1) % 3 + 6
        score_p2 += 1  # index -> weight

    print(score_p1)
    print(score_p2)
