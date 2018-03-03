#Link to problem: https://adventofcode.com/2017/day/1


puzzle_input = input("\nEnter puzzle input: ")
puzzle_len = len(puzzle_input)
print("\nInput puzzle was: " + puzzle_input + '\n')

#PART1
captcha1 = 0

for i in range(0, puzzle_len):

    if i == puzzle_len - 1:
        if puzzle_input[i] == puzzle_input[0]:
            captcha1 = captcha1 + int(puzzle_input[i])
        break        

    if puzzle_input[i] == puzzle_input[i+1]:
        captcha1 = captcha1 + int(puzzle_input[i])
    

#PART2
captcha2 = 0
step = puzzle_len / 2

for i in range(0, puzzle_len):

    if i + step > puzzle_len - 1:

        if puzzle_input[i] == puzzle_input[int((i+step) % puzzle_len)]:
            captcha2 = captcha2 + int(puzzle_input[i])

    else:

        if puzzle_input[i] == puzzle_input[int(i + step)]:
            captcha2 = captcha2 + int(puzzle_input[i])



print("---------RESULTS---------")
print("Captcha1 sum is " + str(captcha1))
print("Captcha2 sum is " + str(captcha2))

        

    




