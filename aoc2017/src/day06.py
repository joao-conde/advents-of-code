input_file = open("input/day06")
memory = input_file.read().split()
input_file.close()

# PART 1 & 2
memory = [int(x) for x in memory]
states = []
cycles = 0
loopLen = 0

while memory not in states:
    # copy the list or it is passed by reference and completes at first attempt (obviously)
    states.append(list(memory))

    blocks = max(memory)
    bankIdx = memory.index(blocks)
    memory[bankIdx] = 0

    for i in range(blocks):
        bankIdx += 1
        if bankIdx >= len(memory):
            bankIdx = 0
        memory[bankIdx] += 1

    cycles += 1

loopLen = len(states) - states.index(memory)

print("Redistribution cycles:", cycles)
print("Redistribution loop length:", loopLen)
