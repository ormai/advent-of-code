def solution(part):
    total = 0
    with open("data/day9.txt") as file:
        for line in file:
            arr = list(map(int, line.split()))
            total += extract(arr, part)
    print(total)


def extract(arr, part):
    if all(x == 0 for x in arr):
        return 0

    differences = [y - x for x, y in zip(arr, arr[1:])]
    return arr[part] + extract(differences, part)
    # arr[-1] is part 1
    # arr[0] is part 2


solution(-1)
solution(0)
