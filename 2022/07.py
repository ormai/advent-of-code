def addPathToDirectories(path, directories):
    if path not in directories.keys():
        directories[path] = 0
    return directories


def solution():
    with open('data/day7.txt') as file:
        sizes = {}
        stack = []
        currentPath = ""
        for line in file:
            if line.startswith("$ cd"):
                if not line.startswith("$ cd ..") and not line.startswith("$ cd /"):
                    currentPath += f"/{line.split()[-1]}" if currentPath != "/" else line.split()[-1]
                    stack.append(currentPath)
                    sizes = addPathToDirectories(currentPath, sizes)

                elif line.strip() == "$ cd /":
                    currentPath = "/"
                    stack = ["/"]
                    sizes = addPathToDirectories(currentPath, sizes)

                elif line.strip() == "$ cd ..":
                    currentPath = "/".join(currentPath.split("/")[:-1])
                    stack.pop()

            if line[0].isdigit():
                file_size = int(line.split()[0])
                for directory in stack:
                    sizes[directory] += file_size

        print("Task 1:", sum(s for s in sizes.values() if s <= 100_000))

        print("Task 2:", sorted(s for s in sizes.values() if s >= sizes['/'] - (70_000_000 - 30_000_000))[0])

solution()
