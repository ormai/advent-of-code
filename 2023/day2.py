def solution():
    sum = 0
    with open("data/day2.txt") as file:
        for line in file:
            cond = True
            id = int(line[5 : line.index(":")])
            cubes = (
                line[line.index(":") + 2 :].strip(" \n").replace(";", ",").split(", ")
            )

            red = green = blue = 0
            for i in range(len(cubes)):
                count, color = cubes[i].split(" ")
                if color == "red":
                    if int(count) > red:
                        red = int(count)
                elif color == "green":
                    if int(count) > green:
                        green = int(count)
                else:
                    if int(count) > blue:
                        blue = int(count)
            sum += red * green * blue
    print(sum)


solution()
