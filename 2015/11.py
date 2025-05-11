from tools import window


def is_valid(pw: list[str]):
    def pair_of_letters(start=0) -> int | None:
        for i in range(start, len(pw) - 1):
            if pw[i] == pw[i + 1]:
                return i + start

    return (
        any(
            tuple(map(lambda c: ord(c) - ord(substr[0]), substr)) == (0, 1, 2)
            for substr in window(pw, 3)
        )
        and (first_pair := pair_of_letters()) is not None
        and pair_of_letters(first_pair + 2) is not None
    )


def next_password(password: str) -> str:
    pw = list(password)
    old_pw = list(pw)
    while not is_valid(pw) or pw == old_pw:
        index = len(pw) - 1
        while pw[index] == "z":
            pw[index] = "a"
            index -= 1
        pw[index] = chr(ord(pw[index]) + (2 if pw[index] in "hnk" else 1))
    return "".join(pw)


def main():
    password = open("input").readline().strip()
    first_next_password = next_password(password)
    print(first_next_password)
    print(next_password(first_next_password))


if __name__ == "__main__":
    main()
