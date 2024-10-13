def main():
    file = open('data/day4.txt')

    count = 0
    # for line in file:
    #     conv = convert(line)
    #     if conv[0] <= conv[2] and conv[1] >= conv[3] or \
    #         conv[2] <= conv[0] and conv[3] >= conv[1]:
    #         count += 1

    for line in file:
        if overlaps(convert(line)):
            count += 1

    print(count)

def convert(stringa):
    data = list(stringa)
    converted = []
    tmp = ''
    for char in data:
        if char in ('-', ',', '\n'):
            converted.append(int(tmp))
            tmp = ''
        else:
            tmp += char

    return converted

def overlaps(pairs):
    first, second = pairs[ : 2], pairs[2 : ]
    range1, range2 = [], []
    
    for i in range(first[0], first[len(first) - 1] + 1): range1.append(i)
    for i in range(second[0], second[len(second) - 1] + 1): range2.append(i)

    for i in range1:
        if i in range2:
            return True
    return False

main()
