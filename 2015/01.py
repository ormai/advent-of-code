floor = position = 0
for i, char in enumerate(open("input").readline()):
    floor += 1 if char == "(" else -1
    if position == 0 and floor == -1:
        position = i
print(floor)
print(position + 1)
