def mySubroutine():
    with open('data/day6.txt') as file:
        line = file.read();
        for i in range(len(line) - 4):
            four = set(line[x] for x in range(i, i + 14))
            if (len(four) == 14):
                print(i + 14)
                break


print(mySubroutine())
