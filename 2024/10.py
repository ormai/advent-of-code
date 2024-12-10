GRID = [list(map(int, line.rstrip())) for line in open("input").readlines()]
score = rating = 0
for r in range(len(GRID)):
    for c in range(len(GRID)):
        if GRID[r][c] == 0:
            visited = [[False for _ in range(len(GRID))] for _ in range(len(GRID))]
            q = [(r, c)]
            while len(q) > 0:
                i, j = q.pop()
                if GRID[i][j] == 9:
                    if not visited[i][j]:
                        visited[i][j] = True
                        score += 1
                    rating += 1
                else:
                    for dj, di in (-1, 0), (0, 1), (1, 0), (0, -1):
                        if (
                            0 <= (ni := di + i) < len(GRID)
                            and 0 <= (nj := dj + j) < len(GRID)
                            and GRID[ni][nj] == GRID[i][j] + 1
                        ):
                            q.append((ni, nj))
print(score, rating, sep="\n")
