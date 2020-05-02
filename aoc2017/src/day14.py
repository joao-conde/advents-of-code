#Link to problem: https://adventofcode.com/2017/day/14

from day10 import knot_hash

puzzle_input = "stpzcrnm"

grid_used = 0
for r in range(128):
  row_input = puzzle_input + "-" + str(r)
  hex_hash = knot_hash(row_input)
  hex_hash = "".join(hex_hash)
  bin_hash = bin(int(hex_hash, 16))[2:]
  prefix = "".join(["0" for _ in range(128 - len(bin_hash))])
  bin128 = prefix + bin_hash
  grid_used += len([b for b in bin128 if b == "1"])

print(grid_used)