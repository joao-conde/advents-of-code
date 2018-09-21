#Link to problem: https://adventofcode.com/2017/day/15


#Python Generator: keyword 'yield' acts as a sort of return ; this function returns one 'generator' object
#When '__next__()' is invoked on the object it computes the new value and returns it, effectively only holding one value in memory at a time
def generator(start_val, factor, divisor):
    x = start_val
    while True:
        x = (x * factor) % divisor
        yield x



#Problem values
divisor = 2147483647
gen_a_factor, gen_b_factor = 16807, 48271

val_a = 512 #int(input("Generator A starting value: "))
val_b = 191 #int(input("Generator B starting value: "))

iterations = 40000000 #40 million


#PART 1

calcA = generator(val_a, gen_a_factor, divisor)
calcB = generator(val_b, gen_b_factor, divisor)
judge_pts = 0

for i in range(iterations):

    val_a = calcA.__next__()
    val_b = calcB.__next__()

    if bin(val_a)[-16:] == bin(val_b)[-16:]:
        judge_pts += 1

print("Judge's final count:", judge_pts)

