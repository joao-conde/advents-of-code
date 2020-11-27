#Link to problem: https://adventofcode.com/2017/day/22

from enum import Enum

src = "../res/d22"
inputFile = open(src)
gridInput = inputFile.read().split('\n')
inputFile.close()

class NodeState(Enum):
    INFECTED = '#'
    CLEAN = '.'
    WEAKENED = 'W'
    FLAGGED = 'F'

class GridCluster:
    def __init__(self, gridInput):
        self.grid = {} # dictionary of (row, col) to cell content, allows for dynamic growth and keeps only cells in path
        for i in range(len(gridInput)):
            for j in range(len(gridInput[i])):
                self.grid[(i,j)] = NodeState.INFECTED if gridInput[i][j] == '#' else NodeState.CLEAN

        self.movementDeltas = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        self.curDelta = 0
        self.newlyInfected = 0

        self.carrierRow = len(gridInput) // 2
        self.carrierCol = len(gridInput[0]) // 2

    def doVirusBurst(self):
        if (self.carrierRow, self.carrierCol) not in self.grid:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.CLEAN # "expand" grid if need be
        self.updateDirection()
        self.updateCurrentNodeState()
        self.moveCarrier()

    def moveCarrier(self):
        self.carrierRow += self.movementDeltas[self.curDelta][0]
        self.carrierCol += self.movementDeltas[self.curDelta][1]

    def updateDirection(self): pass

    def updateCurrentNodeState(self): pass

class GridClusterPart1(GridCluster):
    def updateCurrentNodeState(self):
        if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.CLEAN:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.INFECTED
            self.newlyInfected += 1
        else:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.CLEAN

    def updateDirection(self):
        self.curDelta += 1 if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.INFECTED else -1
        self.curDelta = self.curDelta % 4 if self.curDelta >= 4 else (3 if self.curDelta < 0 else self.curDelta)

class GridClusterPart2(GridCluster):
    def updateCurrentNodeState(self):
        if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.CLEAN:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.WEAKENED
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.WEAKENED:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.INFECTED
            self.newlyInfected += 1
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.INFECTED:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.FLAGGED
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.FLAGGED:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.CLEAN

    def updateDirection(self):
        if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.CLEAN:
            self.curDelta -= 1
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.INFECTED:
            self.curDelta += 1
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.FLAGGED:
            self.curDelta += 2
        self.curDelta = self.curDelta % 4 if self.curDelta >= 4 else (3 if self.curDelta < 0 else self.curDelta)

# PART 1
cluster1 = GridClusterPart1(gridInput)
for _ in range(10000): cluster1.doVirusBurst()
print(f'(Part1) Bursts where a node becomes infected: {cluster1.newlyInfected}')

# PART 2
cluster2 = GridClusterPart2(gridInput)
for _ in range(10000000): cluster2.doVirusBurst()
print(f'(Part2) Bursts where a node becomes infected: {cluster2.newlyInfected}')
