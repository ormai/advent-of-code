ESC, QUOTE = ord("\\"), ord('"')
input = open("input", "rb").readlines()

literal = memory = 0
for string in input:
    literal += len(string) + 2
    i = 0
    while i < len(string):
        if string[i] == ESC:
            if string[i + 1] in [ESC, QUOTE]:
                i += 1
            else:  # "\x00"
                i += 3
        i += 1
        memory += 1
print(literal - memory)

encoded = literal = 0
for string in input:
    literal += len(string)
    encoded += 2
    for c in string:
        if c in [ESC, QUOTE]:
            encoded += 2
        else:
            encoded += 1
print(encoded - literal)
