wrapping_paper = feet_of_ribbon = 0
for present in open("input").readlines():
    l, w, h = [int(d) for d in present.split("x")]
    sides = (l * w, w * h, h * l)
    wrapping_paper += min(sides) + sum(2 * side for side in sides)
    a, b, _ = sorted((l, w, h))
    feet_of_ribbon += a + a + b + b + l * w * h
print(wrapping_paper)  # part one
print(feet_of_ribbon)  # part two
