#Link to problem: https://adventofcode.com/2017/day/15


#Python Generator: keyword 'yield' acts as a sort of return ; this function returns one 'generator' object
#When '__next__()' is invoked on the object it computes the new value and returns it, effectively only holding one value in memory at a time
def generator(start_val, factor, divisor, multiple = 1):
    x = start_val
    while True:
        x = (x * factor) % divisor
        
        if x % multiple == 0:
            yield x


#Problem values
divisor = 2147483647
gen_a_factor, gen_b_factor = 16807, 48271

init_val_a = 512 #int(input("Generator A starting value: "))
init_val_b = 191 #int(input("Generator B starting value: "))

#PART 1

val_a = init_val_a
val_b = init_val_b
iterations_p1 = 40000000 #40 million

calcA_p1 = generator(val_a, gen_a_factor, divisor)
calcB_p1 = generator(val_b, gen_b_factor, divisor)

judge_pts_p1 = 0

for i in range(iterations_p1):

    val_a = calcA_p1.__next__()
    val_b = calcB_p1.__next__()

    if bin(val_a)[-16:] == bin(val_b)[-16:]:
        judge_pts_p1 += 1


#PART 2

val_a = init_val_a
val_b = init_val_b
iterations_p2 = 5000000 #5 million

multiple_of_A, multiple_of_B = 4, 8

calcA_p2 = generator(val_a, gen_a_factor, divisor, multiple_of_A)
calcB_p2 = generator(val_b, gen_b_factor, divisor, multiple_of_B)

judge_pts_p2 = 0


for i in range(iterations_p2):

    val_a = calcA_p2.__next__()
    val_b = calcB_p2.__next__()

    if bin(val_a)[-16:] == bin(val_b)[-16:]:
        judge_pts_p2 += 1


#Results
print("\nOld generator judge's final count:", judge_pts_p1)
print("New generator judge's final count:", judge_pts_p2)
