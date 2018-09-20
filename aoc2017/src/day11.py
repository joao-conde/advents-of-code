#Link to problem: https://adventofcode.com/2017/day/11

#Useful material on Hex grids: https://www.redblobgames.com/grids/hexagons/

#For current repos config path is '../res/d11input.txt'
src = input("Input file path + extension (e.g.: /dir/file.txt): ")

input_file = open(src)
directions = input_file.read().split(',')
input_file.close()


#PART 1 & 2

#distance between A and B in a hex grid (with cube coordinates)
def hex_grid_distance(a, b):
    return (abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2])) // 2

#Hex grid coord system
hex_directions = {
    "n":  [0, 1, -1],
    "s":  [0, -1, 1],
    "nw": [-1, 1, 0],
    "se": [1, -1, 0],
    "ne": [1, 0, -1],
    "sw": [-1, 0, 1]
}

pos = [0, 0, 0]
dists = []

for dir in directions:
    pos[0] += hex_directions[dir][0]
    pos[1] += hex_directions[dir][1]
    pos[2] += hex_directions[dir][2]

    dists.append(hex_grid_distance(pos, [0, 0, 0]))

print("\nFewest number of steps required:", hex_grid_distance(pos, [0, 0, 0]))
print("\nFurthest from starting position:", max(dists))
