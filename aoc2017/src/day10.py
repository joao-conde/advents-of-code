#Link to problem: https://adventofcode.com/2017/day/10

#Reverse a portion (sublist) of a list
def reverse_sublist(origin_list, start, end):
    sublist = origin_list[start:end]
    reverse_sublist = sublist[::-1]
    origin_list[start:end] = reverse_sublist


lengths = input("Enter lengths: ")
lengths = lengths.split(',')
lengths = [int(length) for length in lengths]

circ_list = []
for i in range(0,5) : circ_list.append(i)

pos = skip = 0

for length in lengths:

    if length > len(circ_list) : continue

    if pos > len(circ_list) : pos = 0

    if pos + length > len(circ_list) :
        excess = (pos + length) - len(circ_list)
        print(length, excess)

    reverse_sublist(circ_list, pos, pos + length)
    pos += length + skip
    skip += 1

print(circ_list)
