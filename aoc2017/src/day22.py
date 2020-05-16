#Link to problem: https://adventofcode.com/2017/day/22

src = "../res/d22"
inputFile = open(src)
gridInput = inputFile.read().split('\n')
inputFile.close()

# PART 1
INFECTED, CLEAN = '#', '.'
carrierRow, carrierCol = len(gridInput) // 2, len(gridInput[0]) // 2 
grid = {} # dictionary of (row, col) to cell content, allows for dynamic growth and keeps only cells in path
deltas, curDir = [(-1, 0), (0, 1), (1, 0), (0, -1)], 0 # up, right, down, left, up, right, ...
newlyInfected = 0
for i in range(len(gridInput)): 
    for j in range(len(gridInput[i])): 
        grid[(i,j)] = gridInput[i][j]

for _ in range(10000):
    if (carrierRow, carrierCol) not in grid: 
        grid[(carrierRow, carrierCol)] = CLEAN

    curDir += 1 if grid[(carrierRow, carrierCol)] == INFECTED else -1
    curDir = curDir % 4 if curDir >= 4 else (3 if curDir < 0 else curDir)

    if grid[(carrierRow, carrierCol)] == CLEAN:
        grid[(carrierRow, carrierCol)] = INFECTED
        newlyInfected += 1
    else:
        grid[(carrierRow, carrierCol)] = CLEAN

    carrierRow += deltas[curDir][0]
    carrierCol += deltas[curDir][1]
 
print(f'(Part1) Bursts where a node becomes infected: {newlyInfected}')
    