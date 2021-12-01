from collections import deque

input_file = open("input/day17")
step = int(input_file.read())
input_file.close()


def spin(insertions, step):
    spinlock = deque([0])
    for i in range(insertions):
        spinlock.rotate(-step)
        spinlock.append(i + 1)
    return spinlock


# PART 1 & 2
spinlock1 = spin(2017, step)
print("Value after 2017:", spinlock1[0])

spinlock2 = spin(50_000_000, step)
print("Value after 0:", spinlock2[spinlock2.index(0) + 1])
