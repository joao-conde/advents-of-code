input_file = open("input/day01")
puzzle_input = input_file.read()
input_file.close()

puzzle_len = len(puzzle_input)

# PART1
captcha1 = 0

for i in range(0, puzzle_len):
    # last element has to check the first (circular list)
    if i == puzzle_len - 1:
        if puzzle_input[i] == puzzle_input[0]:
            captcha1 = captcha1 + int(puzzle_input[i])
        break

    if puzzle_input[i] == puzzle_input[i + 1]:
        captcha1 = captcha1 + int(puzzle_input[i])

print("Captcha1 sum is " + str(captcha1))

# PART2
captcha2 = 0
step = puzzle_len / 2

for i in range(0, puzzle_len):
    # past the last element check has to loop from beginning
    if i + step > puzzle_len - 1:
        if puzzle_input[i] == puzzle_input[int((i + step) % puzzle_len)]:
            captcha2 = captcha2 + int(puzzle_input[i])
    else:
        if puzzle_input[i] == puzzle_input[int(i + step)]:
            captcha2 = captcha2 + int(puzzle_input[i])

print("Captcha2 sum is " + str(captcha2))
