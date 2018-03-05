#Link to problem: https://adventofcode.com/2017/day/3

import math

#PART1

#Manhattan distance algorithm
def calcManhattanDist(originRow, originCol, destRow, destCol):
    distance = 0
    
    while originRow != destRow:
        if(originRow < destRow) : destRow -= 1 
        else: destRow += 1
        
        distance += 1

    while originCol != destCol:
        if(originCol < destCol) : destCol -=1 
        else: destCol +=1
        
        distance += 1

    return distance


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

center = math.floor(gridSize / 2)
posX = posY = center
grid[posX][posY] = 1

element = 2
control = 0


#Grid filling algorithm
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
    


#Find element coords
elX = elY = 0
for i in range(gridSize):
    for j in range(gridSize):
        if grid[i][j] == puzzInput:
            elX = i
            elY = j
            break

#Calculate distance from point to origin (1)
distance = calcManhattanDist(center, center, elX, elY) 


#PART2


#Print grid
print("\n--------Created following grid--------")
for line in grid:
    print(line)

print("\nDistance: " + str(distance))




