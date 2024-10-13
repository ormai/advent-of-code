def main():
    file = open('data/day3.txt')

    data = convertInList(file)

    add = 0
    # First part
    # for line in file:
    #     halves = getHalves(line)
    #     itemType = findDuplicate(halves[0], halves[1])
    #     priority = getPriority(itemType)
    #     add += priority

    # Second part
    for i in range(0, len(data), 3):
        if i + 2 < len(data):
            itemType = findType(data[i], data[i + 1], data[i + 2])
            priority = getPriority(itemType)
            add += priority

    print(add)

def convertInList(f):
    result = []

    for line in f:
        lineConv = list(line)
        lineConv.pop(len(lineConv) - 1)
        result.append(lineConv)

    return result

def getHalves(l):
    adj = list(l)
    adj2 = []
    adj.pop(len(adj) - 1)
    
    point = len(adj) // 2

    for _ in range(point, len(adj)):
        adj2.append(adj.pop(point))

    return [adj, adj2]


def findDuplicate(list1, list2):
    for i in range(len(list1)):
        for j in range(len(list2)):
            if list1[i] == list2[j]:
                return list1[i]

def findType(list1, list2, list3):
    for i in range(len(list1)):
        for j in range(len(list2)):
            for k in range(len(list3)):
                if list1[i] == list2[j] == list3[k]:
                    return list1[i]

def getPriority(char):
    charCode = ord(char)

    result = 0

    if char.islower(): # priority from 1 through 26
        result = charCode - 96
    elif char.isupper(): # priority from 27 through 52
        result = charCode - 38

    return result

main()
