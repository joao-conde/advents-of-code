#Link to problem: https://adventofcode.com/2017/day/3

import math


def calcManhattanDist(originRow, originCol, destRow, destCol):
    print(originRow + originCol + destRow + destCol)

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
# 1xR + (2*i+1)xU + (U+1)xL + (U+1)xD + (U+1)xR

posX = posY = math.floor(gridSize / 2)
grid[posX][posY] = 1

element = 2
control = 0

while element <= puzzInput:
    
    upTimes = 2*control + 1

    #one right and fill
    posY +=1
    grid[posX][posY] = element

    element += 1
    if element > puzzInput : break

    #up times and fill for each
    for i in range(upTimes):
        posX -= 1
        grid[posX][posY] = element

        element += 1
        if element > puzzInput : break

    if element > puzzInput : break


    #left times and fil for each

    for i in range(upTimes + 1):
        posY -= 1
        grid[posX][posY] = element

        element += 1
        if element > puzzInput : break

    if element > puzzInput : break


    #down times and fill for each

    for i in range(upTimes + 1):
        posX += 1
        grid[posX][posY] = element

        element += 1
        if element > puzzInput : break

    if element > puzzInput : break

    #right times and fill for each

    for i in range(upTimes + 1):
        posY += 1
        grid[posX][posY] = element

        element += 1
        if element > puzzInput : break


    control += 1
    if element > puzzInput : break
    


#2 linha 1 coluna
#grid[line][col]
#grid[1][0] = 3

for line in grid:
    print(line)