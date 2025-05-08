def look_and_say(seq: list[int]) -> list[int]:
    index = 0
    current = seq[index]
    result = []
    for i, n in enumerate(seq + [0]):
        if n != current:
            result.append(i - index)
            result.append(current)
            current = n
            index = i
    return result


input = list(map(int, list(open("input").readline().strip())))
for length in 40, 50:
    s = input
    for _ in range(length):
        s = look_and_say(s)
    print(len(s))
