#Link to problem: https://adventofcode.com/2017/day/19

def update_direction(lines: list, row: int, col: int):
    for r in range(-1, 2):
        for c in range(-1, 2):
            if r == 0 and c == 0: continue
            if row + r >= len(lines) or col + c >= len(lines[row + r]): continue
            if row + r < 0 or col + c < 0: continue
            if lines[row + r][col + c] != " " and not (row + r == row - dr and col + c == col - dc):
                return r, c

src = "../res/d19"
input_file = open(src)
lines = input_file.read().split("\n")
input_file.close()

# PARTS 1 & 2
row = 0
col = lines[row].find("|")
dr, dc = 1, 0

letters, steps = [], 0
while True:
    cur = lines[row][col]
    if cur == " ": break

    if cur == "+":
        dr, dc = update_direction(lines, row, col)
    elif cur.isalpha():
        letters.append(cur)

    row += dr
    col += dc
    steps += 1

print(f'(Part1) Packet sees {"".join(letters)}')
print(f'(Part2) Packet takes {steps} steps')
