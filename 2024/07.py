from operator import add, mul
from itertools import product
from functools import reduce

# It's a bit slow


def total_calibration(operators):
    acc = 0
    with open("input") as file:
        for line in file:
            val, *nums = map(int, line.strip().replace(":", "").split())
            for ops in product(operators, repeat=len(nums) - 1):
                expr = zip(ops, nums[1:])
                if reduce(lambda a, x: x[0](a, x[1]), expr, nums[0]) == val:
                    acc += val
                    break
    return acc


print(total_calibration((add, mul)))
print(total_calibration((add, mul, lambda a, b: int(str(a) + str(b)))))
