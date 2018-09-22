#Link to problem: https://adventofcode.com/2017/day/17

#PART 1

insertions = 2017
step = 359 #int(input("Step size: "))

cbuf, curr_pos = [0], 0

for i in range(insertions):
    
    pos = (step % len(cbuf))
    pos = (pos + curr_pos) % len(cbuf)
    cbuf.insert(pos + 1, i+1)
    curr_pos = pos + 1

print("Element after 2017:", cbuf[cbuf.index(2017)+1])



