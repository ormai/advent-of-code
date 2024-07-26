def deliver(directions, santa_count):
    pos = [[0, 0] for _ in range(santa_count)]
    cur = 0  # which santa
    visited = set(["00"])
    for c in directions:
        match c:
            case ">":
                pos[cur][0] += 1
            case "<":
                pos[cur][0] -= 1
            case "^":
                pos[cur][1] += 1
            case "v":
                pos[cur][1] -= 1
        visited.add(str(pos[cur][0]) + str(pos[cur][1]))
        cur = (cur + 1) % santa_count
    return len(visited)


with open("input") as input:
    line = input.readline()
    print(deliver(line, 1))
    print(deliver(line, 2))
