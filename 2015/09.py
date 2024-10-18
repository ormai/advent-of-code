# https://en.wikipedia.org/wiki/Travelling_salesman_problem

from itertools import permutations
from sys import maxsize

with open("input") as input:
    distances: dict[str, dict[str, int]] = {}

    # Since the number of cities is little, this instance of the problem can be
    # solved by an exact algoritm that will take the minimum distance of all
    # the possible ordered permutatins of the cities.
    # The runtime will be n!, where n = len(cities)
    cities = set()

    for distance in input.readlines():
        a, _, b, _, d = distance.split()
        if a not in distances:
            distances[a] = {}
        distances[a][b] = int(d)
        cities.add(a)
        cities.add(b)


    min_distance = maxsize
    max_distance = 0
    for permutation in permutations(cities):
        distance = 0
        for i in range(len(permutation) - 1):
            # The graph is fully connected and since distances are valid both
            # ways, it is not directed.
            a, b = permutation[i : i + 2]
            if a in distances and b in distances[a]:
                distance += distances[a][b]  # from a to b
            else:
                distance += distances[b][a]  # ... or from b to a
        if min_distance > distance:
            min_distance = distance
        elif max_distance < distance:
            max_distance = distance
    print(min_distance)
    print(max_distance)
