def deliver(directions: str, santa_count: int) -> int:
    pos = [(0, 0) for _ in range(santa_count)]
    santa = 0
    visited = set([(0, 0)])
    for c in directions:
        x, y = pos[santa]
        match c:
            case ">":
                pos[santa] = x + 1, y
            case "<":
                pos[santa] = x - 1, y
            case "^":
                pos[santa] = x, y + 1
            case "v":
                pos[santa] = x, y - 1
        visited.add(pos[santa])
        santa = (santa + 1) % santa_count
    return len(visited)


with open("input") as input:
    line = input.readline()
    print(deliver(line, 1))
    print(deliver(line, 2))
