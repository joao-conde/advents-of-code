#Link to problem: https://adventofcode.com/2017/day/5

import sys

#For current repos config path is '../res/d05.txt'

src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
offsets = input_file.read().split('\n')

input_file.close()

offsets = [int(n) for n in offsets]

#forcing a list copy so when one is changed I still have the original
offsets2 = list(offsets)

print("\nWait a few seconds. Calculating steps...\n")
sys.stdout.flush() #forcing stdout flush since for some reason it held the print while calculating result below

#PART1

index1 = 0
steps1 = 0
while index1 < len(offsets):
    offset = offsets[index1]
    offsets[index1] += 1
    index1 += offset
    steps1 += 1
    

#PART2

index2 = 0
steps2 = 0

while index2 < len(offsets2):
    offset = offsets2[index2]
    
    if offset >= 3 : 
        offsets2[index2] -= 1
    else:
        offsets2[index2] += 1
    
    index2 += offset
    steps2 += 1


#Results print
print("Part1 steps:", steps1)
print("Part2 steps:", steps2)