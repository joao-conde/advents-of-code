#Link to problem: https://adventofcode.com/2017/day/10

def reverse_portion_circular_list(l, start, end):
    from_pos = [x % len(l) for x in range(start, end)]
    to_pos = [x % len(l) for x in range(start, end)][::-1]
    num_swaps = len(from_pos) // 2
    for i in range(num_swaps):
        if from_pos[i] == to_pos[i]: 
            return l
        else:
            tmp = l[from_pos[i]]
            l[from_pos[i]] = l[to_pos[i]]
            l[to_pos[i]] = tmp
    return l

src = "../res/d10"
input_file = open(src)
lengths = input_file.read().split(',')
lengths = [int(length) for length in lengths]

circular_list = [x for x in range(256)]
pos = skip = 0
for length in lengths:
    circular_list = reverse_portion_circular_list(circular_list, pos, pos + length)
    pos = (pos + length + skip) % len(circular_list)
    skip += 1

print(f'(Part1) Result of multiplying the first two numbers in the list: {circular_list[0] * circular_list[1]}')
