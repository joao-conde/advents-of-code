input_file = open("input/day15")
lines = input_file.readlines()
input_file.close()

init_val_a = int(lines[0].split()[-1])
init_val_b = int(lines[1].split()[-1])


def generator(start_val, factor, divisor, multiple=1):
    x = start_val
    while True:
        x = (x * factor) % divisor
        if x % multiple == 0:
            yield x


print(
    "Wait a moment. Computing judge points for both parts, this may take several minutes..."
)

# PART 1
val_a = init_val_a
val_b = init_val_b
iterations_p1 = 40_000_000
divisor = 2147483647
gen_a_factor, gen_b_factor = 16807, 48271

calc_a_p1 = generator(val_a, gen_a_factor, divisor)
calc_b_p1 = generator(val_b, gen_b_factor, divisor)

judge_pts_p1 = 0
for i in range(iterations_p1):
    val_a = calc_a_p1.__next__()
    val_b = calc_b_p1.__next__()
    if bin(val_a)[-16:] == bin(val_b)[-16:]:
        judge_pts_p1 += 1

print("Old generator judge's final count:", judge_pts_p1)

# PART 2
val_a = init_val_a
val_b = init_val_b
iterations_p2 = 5_000_000

multiple_of_a, multiple_of_b = 4, 8

calcA_p2 = generator(val_a, gen_a_factor, divisor, multiple_of_a)
calcB_p2 = generator(val_b, gen_b_factor, divisor, multiple_of_b)

judge_pts_p2 = 0
for i in range(iterations_p2):
    val_a = calcA_p2.__next__()
    val_b = calcB_p2.__next__()

    if bin(val_a)[-16:] == bin(val_b)[-16:]:
        judge_pts_p2 += 1

print("New generator judge's final count:", judge_pts_p2)
