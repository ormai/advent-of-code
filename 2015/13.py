from itertools import permutations


def change_in_happiness(
    invited: dict[str, dict[str, int]], arragement: tuple[str, ...]
) -> int:
    return sum(
        invited[arragement[i]].get(arragement[(i + dir) % len(arragement)], 0)
        for i in range(len(arragement))
        for dir in (1, -1)
    )


def optimal_seating_arrangement(invited: dict[str, dict[str, int]]) -> int:
    return max(
        change_in_happiness(invited, arragement)
        for arragement in permutations(invited)
    )


def main():
    invited: dict[str, dict[str, int]] = {}
    for line in open("input").readlines():
        person, _, sign, points, *_, neighbor = line.rstrip("\n.").split()
        points = int(points)
        if sign == "lose":
            points *= -1
        invited.setdefault(person, {})[neighbor] = points

    print(optimal_seating_arrangement(invited))  # part one

    invited["Mario"] = {}
    print(optimal_seating_arrangement(invited))  # part two


if __name__ == "__main__":
    main()
