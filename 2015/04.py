from hashlib import md5

SECRET_KEY = "yzbqklnj"


def mine(zeroes: int) -> int:
    i = 1
    while True:
        m = md5()
        m.update(f"{SECRET_KEY}{i}".encode())
        if m.hexdigest().startswith("0" * zeroes):
            return i
        i += 1


print(mine(5))
print(mine(6))
