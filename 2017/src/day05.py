import sys

input_file = open("input/day05")
offsets = input_file.read().split("\n")
input_file.close()

offsets = [int(n) for n in offsets]
offsets2 = list(offsets)

print("Wait a few seconds. Calculating steps...")
sys.stdout.flush()

# PART1
index1 = 0
steps1 = 0
while index1 < len(offsets):
    offset = offsets[index1]
    offsets[index1] += 1
    index1 += offset
    steps1 += 1

print("Part1 steps:", steps1)

# PART2
index2 = 0
steps2 = 0
while index2 < len(offsets2):
    offset = offsets2[index2]

    if offset >= 3:
        offsets2[index2] -= 1
    else:
        offsets2[index2] += 1

    index2 += offset
    steps2 += 1

print("Part2 steps:", steps2)
