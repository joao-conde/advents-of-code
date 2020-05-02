#Link to problem: https://adventofcode.com/2017/day/13

src = '../res/d13'

input_file = open(src)
layers = input_file.read().split("\n")
input_file.close()

layers = [layer.replace(':', '').split() for layer in layers]
layers_dict = {}
for pair in layers:
    layers_dict[int(pair[0])] = int(pair[1])


# PART 1
packet_pos, severity = 0, 0
max_ps = max([x for x in layers_dict])
for ps in range(max_ps + 1): 
    if ps in layers_dict:
        if ps % (2*(layers_dict[ps] - 1)) == 0: # scanner is at position 0 at frequency 2 * (range - 1)
            severity += (ps * layers_dict[ps])
    
    packet_pos += 1

print("(Part1) Severity of the whole trip:", severity)


# PART 2

