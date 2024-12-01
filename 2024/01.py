with open("input") as input:
    n = [int(n) for line in input for n in line.split()]

left, right = n[:-1:2], n[1::2]

left_s, right_s = sorted(left), sorted(right)
print(sum(abs(left_s[i] - right_s[i]) for i in range(len(left_s))))

print(sum(n * right.count(n) for n in left))
