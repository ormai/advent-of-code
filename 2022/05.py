def main():
    commands = []
    lines = []
    with open('data/day5.txt') as file:
        for line in file:
            if line.startswith('move'):
                command = []
                tmp = list(line)
                curr = ''
                for i in tmp:
                    if i.isdigit():
                        curr += i
                    else:
                        if curr != '':
                            command.append(int(curr))
                        curr = ''
                commands.append(command)
            else:
                if '[' in line:
                    raw = []
                    for i in range(1, len(line), 4):
                        raw.append(line[i])
                    lines.append(raw)

    stacks = createStacks(lines)
    # for i in stacks: print(i)
    for i in commands:
        tmp = []
        for _ in range(i[0]):
            tmp.append(stacks[i[1] - 1].pop(0))
        for _ in range(i[0]):
            stacks[i[2] - 1].insert(0, tmp.pop())

    for i in range(len(stacks)): 
        print(stacks[i])

def createStacks(M):
    stacks = []
    for j in range(len(M[0])):
        stacks.append([])
        for i in range(len(M)):
            stacks[j].append(M[i][j])

    for i in range(len(stacks)):
        while ' ' in stacks[i]:
            stacks[i].remove(' ')

    return stacks

main()
