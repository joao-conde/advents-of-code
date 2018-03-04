#Link to problem: https://adventofcode.com/2017/day/3

import math

puzzInput = int(input("Enter your puzzle input: "))

#1 gridSize is always odd so it has a middle for the '1' and 
#big enough to contain all numbers
gridSize = math.ceil(math.sqrt(puzzInput))

if(gridSize % 2 == 0) : gridSize += 1

#2 Create a square grid of gridSize by gridSize filled with 0's
grid = [[0 for element in range(gridSize)] for line in range(gridSize)]


#3 The filling pattern is as it follows:
#From a starting point, in order to complete one spiral (one cycle)
#you use the following algorithm (found by pattern analysis)
#being R(right), U(p), D(own), L(eft)
# 1xR + (2*i+1)xU + (U+1)xL + (U+1)xD + (U+2)xR

#middle position for the 1
posX = posY = math.floor(gridSize/2)
element = 1
i = 0

while element <= puzzInput:
    
    #fill center
    grid[posY][posX] = element
    element += 1
    if element >= puzzInput : break

    #right move
    posX += 1
    grid[posY][posX] = element
    element += 1
    if element >= puzzInput : break

    upTimes = 2*i + 1

    #up moves
    for i in range(upTimes):
        posY -= 1
        grid[posY][posX] = element
        element += 1
        if element >= puzzInput : break
       

    #left moves
    for i in range(upTimes + 1):
        posX -= 1
        grid[posY][posX] = element
        element += 1
        if element >= puzzInput : break

    #down moves
    for i in range(upTimes + 1):
        posY += 1
        grid[posY][posX] = element
        element += 1
        if element >= puzzInput : break

    #right moves
    for i in range(upTimes + 1):
        posX += 1
        grid[posY][posX] = element
        element += 1

    i += 1






#2 linha 1 coluna
#grid[1][0] = 3

for line in grid:
    print(line)