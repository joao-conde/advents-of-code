#Link to problem: https://adventofcode.com/2017/day/18

def get_value(a, registers):
    #input can be a register letter -> recover register value
    try:
        value = int(a)
    except ValueError:
        value = registers[a]

    return value

#For current repos config path is '../res/d18.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")

src = "../res/d18.txt"

input_file = open(src)
instructions = input_file.read().split('\n')
input_file.close()

#init register from a to z as 0
registers = {}
for i in range(26): registers[chr(ord('a')+i)] = 0

running, instr_ptr, last_played = True, 0, 0
while running:

    jump_len = 1
    instr = instructions[instr_ptr].split()
    
    if "snd" in instr:
        last_played = get_value(instr[1], registers)
    
    elif "rcv" in instr:
        if get_value(instr[1], registers) != 0:
            print("\nRecovered frequency value:", last_played)
            running = False

    elif "set" in instr:
        registers[instr[1]] = get_value(instr[2], registers)

    elif "add" in instr:
        registers[instr[1]] += get_value(instr[2], registers)

    elif "mul" in instr:
        registers[instr[1]] *= get_value(instr[2], registers)

    elif "mod" in instr:
        registers[instr[1]] %= get_value(instr[2], registers)

    #instruction pointer update
    if "jgz" in instr:
        if get_value(instr[1], registers) > 0:
            jump_len = get_value(instr[2], registers)
    
    instr_ptr += jump_len


