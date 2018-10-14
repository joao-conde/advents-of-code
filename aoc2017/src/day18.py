#Link to problem: https://adventofcode.com/2017/day/18

def get_value(a, registers):
    #input can be a register letter -> recover register value
    try:
        value = int(a)
    except ValueError:
        value = registers[a]

    return value


def do_regular_ops(instr, registers):
    jump_len = 1

    if "set" in instr:
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

    return jump_len

def init_registers():
    registers = {}
    for i in range(26): registers[chr(ord('a')+i)] = 0
    return registers

#For current repos config path is '../res/d18.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")

src = "../res/d18.txt"

input_file = open(src)
instructions = input_file.read().split('\n')
input_file.close()

#PART 1 

#init register from a to z as 0
registers = init_registers()
running, instr_ptr, last_played = True, 0, 0
while instr_ptr >= 0 and instr_ptr < len(instructions) and running:
    instr = instructions[instr_ptr].split()
    
    if "snd" in instr:
        last_played = get_value(instr[1], registers)
    
    elif "rcv" in instr:
        if get_value(instr[1], registers) != 0:
            print("\nRecovered frequency value:", last_played)
            running = False
    
    instr_ptr += do_regular_ops(instr, registers)


#PART 2

#program 0 and 1
p0_state, p0_queue, p0_registers, p0_instr_ptr = "RDY", [], init_registers(), 0
p1_state, p1_queue, p1_registers, p1_instr_ptr = "RDY", [], init_registers(), 0

p0_registers['p'], p1_registers['p'] = 0, 1

p1_sent_msg = 0
while p0_state != "RCV" or p1_state != "RCV":

    if p0_state != "RCV":
        #cicle of program 0
        while p0_instr_ptr >= 0 and p0_instr_ptr < len(instructions) and p0_state != "RCV":
            instr = instructions[p0_instr_ptr].split()
            
            if "snd" in instr:
                p1_queue.append(get_value(instr[1], p0_registers))
            
            elif "rcv" in instr:
                p0_state = "RCV"
                if len(p0_queue) > 0:
                    p0_registers[instr[1]] = p0_queue[0]
                    p0_queue.pop()
                    p0_state = "RDY"
                else: 
                    continue
                
            p0_instr_ptr += do_regular_ops(instr, p0_registers)

    else:
        #cicle of program 1
        while p1_instr_ptr >= 0 and p1_instr_ptr < len(instructions) and p1_state != "RCV":
            instr = instructions[p1_instr_ptr].split()
            
            if "snd" in instr:
                p0_queue.append(get_value(instr[1], p1_registers))
                p1_sent_msg += 1
            
            elif "rcv" in instr:
                p1_state = "RCV"
                if len(p1_queue) > 0:
                    p1_registers[instr[1]] = get_value(p1_queue[0], p1_registers)
                    p1_queue.pop()
                    p1_state = "RDY"
                else: 
                    continue
                
            p1_instr_ptr += do_regular_ops(instr, p1_registers)

print("\nProgram 1 sent", p1_sent_msg, "messages")