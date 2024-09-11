from enum import Enum

input_file = open("input/day22")
grid_input = input_file.read().split("\n")
input_file.close()


class NodeState(Enum):
    INFECTED = "#"
    CLEAN = "."
    WEAKENED = "W"
    FLAGGED = "F"


class GridCluster:
    def __init__(self, grid_input):
        self.grid = (
            {}
        )  # dictionary of (row, col) to cell content, allows for dynamic growth and keeps only cells in path
        for i in range(len(grid_input)):
            for j in range(len(grid_input[i])):
                self.grid[(i, j)] = (
                    NodeState.INFECTED if grid_input[i][j] == "#" else NodeState.CLEAN
                )

        self.movementDeltas = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        self.curDelta = 0
        self.newlyInfected = 0

        self.carrierRow = len(grid_input) // 2
        self.carrierCol = len(grid_input[0]) // 2

    def do_virus_burst(self):
        if (self.carrierRow, self.carrierCol) not in self.grid:
            self.grid[(self.carrierRow, self.carrierCol)] = (
                NodeState.CLEAN
            )  # "expand" grid if need be
        self.update_direction()
        self.update_current_node_state()
        self.move_carrier()

    def move_carrier(self):
        self.carrierRow += self.movementDeltas[self.curDelta][0]
        self.carrierCol += self.movementDeltas[self.curDelta][1]

    def update_direction(self):
        pass

    def update_current_node_state(self):
        pass


class GridClusterPart1(GridCluster):
    def update_current_node_state(self):
        if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.CLEAN:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.INFECTED
            self.newlyInfected += 1
        else:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.CLEAN

    def update_direction(self):
        self.curDelta += (
            1
            if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.INFECTED
            else -1
        )
        self.curDelta = (
            self.curDelta % 4
            if self.curDelta >= 4
            else (3 if self.curDelta < 0 else self.curDelta)
        )


class GridClusterPart2(GridCluster):
    def update_current_node_state(self):
        if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.CLEAN:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.WEAKENED
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.WEAKENED:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.INFECTED
            self.newlyInfected += 1
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.INFECTED:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.FLAGGED
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.FLAGGED:
            self.grid[(self.carrierRow, self.carrierCol)] = NodeState.CLEAN

    def update_direction(self):
        if self.grid[(self.carrierRow, self.carrierCol)] == NodeState.CLEAN:
            self.curDelta -= 1
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.INFECTED:
            self.curDelta += 1
        elif self.grid[(self.carrierRow, self.carrierCol)] == NodeState.FLAGGED:
            self.curDelta += 2
        self.curDelta = (
            self.curDelta % 4
            if self.curDelta >= 4
            else (3 if self.curDelta < 0 else self.curDelta)
        )


# PART 1
cluster1 = GridClusterPart1(grid_input)
for _ in range(10000):
    cluster1.do_virus_burst()
print(f"Bursts where a node becomes infected: {cluster1.newlyInfected}")

# PART 2
cluster2 = GridClusterPart2(grid_input)
for _ in range(10000000):
    cluster2.do_virus_burst()
print(f"Bursts where a node becomes infected: {cluster2.newlyInfected}")
