with open("input") as input:
    grid_one = [[False for _ in range(1000)] for _ in range(1000)]
    grid_two = [[0 for _ in range(1000)] for _ in range(1000)]
    for instruction in input.readlines():
        action, begin, _, end = instruction.removeprefix("turn ").split()
        begin_x, begin_y = map(int, begin.split(","))
        end_x, end_y = map(int, end.split(","))

        for x in range(begin_x, end_x + 1):
            for y in range(begin_y, end_y + 1):
                if action == "on":
                    grid_one[x][y] = True
                    grid_two[x][y] += 1
                elif action == "off":
                    grid_one[x][y] = False
                    if grid_two[x][y] > 0:
                        grid_two[x][y] -= 1
                else:
                    grid_one[x][y] = not grid_one[x][y]
                    grid_two[x][y] += 2
    print(sum(sum(row) for row in grid_one))
    print(sum(sum(row) for row in grid_two))
