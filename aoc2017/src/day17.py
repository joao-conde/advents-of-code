#Link to problem: https://adventofcode.com/2017/day/17

#deque acts as a list but with better performance
#list incur O(n) memory movement costs for pop and insert
#deque's guarantee O(1)
from collections import deque

def spin(insertions, step):
    spinlock = deque([0])
    for i in range(insertions):
        spinlock.rotate(-step)
        spinlock.append(i+1)
    return spinlock


#PART 1 & 2

print("\nWait a few seconds. Computing results...\n")

step = int(input("Spin step size: "))
spinlock1 = spin(2017, step)
spinlock2 = spin(50_000_000, step)

print("\nValue after 2017:", spinlock1[0])
print("Value after 0:", spinlock2[spinlock2.index(0) + 1])



