from math import gcd

# first, second, all other lines
instructions, _, *nodes = open("data/day8.txt").read().splitlines()

network = {}
for node in nodes:
    label, next = node.split(" = ")
    network[label] = next[1:-1].split(", ")

start = [key for key in network if key.endswith("A")]
distances = []  # How many steps for each node to reach a node ending with Z

for node in start:
    i = 0  # index to cycle trough the instructions
    steps = 0  # count the steps

    while not node.endswith("Z"):
        if i == len(instructions):
            i = 0
        node = network[node][0 if instructions[i] == "L" else 1]
        i += 1
        steps += 1

    distances.append(steps)

# combine the distances
lcm = distances.pop()
for n in distances:
    lcm = lcm * n // gcd(lcm, n)

print(lcm)
