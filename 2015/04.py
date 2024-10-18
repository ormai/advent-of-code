from hashlib import md5

secret_key = "yzbqklnj"


def mine(zeroes):
    i = 1
    while True:
        message = (secret_key + str(i)).encode()
        m = md5()
        m.update(message)
        if m.hexdigest().startswith("0" * zeroes):
            return i
        i += 1


print(mine(5))
print(mine(6))
