#Link to problem: https://adventofcode.com/2017/day/6


#For current repos config path is '../res/d5input.txt'
src = '../res/d6input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
memory = input_file.read().split()
input_file.close()

memory = [int(x) for x in memory]

#PART1

states = []
cycles = 0

while memory not in states:
    #copy the list or it is passed by reference and completes at first attempt (obviously)
    states.append(list(memory))

    blocks = max(memory)
    bankIdx = memory.index(blocks)
    memory[bankIdx] = 0

    for i in range(blocks):
        bankIdx += 1
        if bankIdx >= len(memory) : bankIdx = 0
        memory[bankIdx] += 1
    
    cycles += 1


print("Current memory state:", memory)
print("Redistribution cycles:", cycles)


