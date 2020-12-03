#Link to problem: https://adventofcode.com/2017/day/23

from day18 import Program

class CoprocessorProgram(Program):
    def __init__(self, registers, instructions = [], instr_ptr = 0):
        super().__init__(registers, instructions, instr_ptr)
        self.mult_instr_cnt = 0

    def execute_instr(self, instr):
        jump_len = 1

        if "set" in instr:
            self.execute_set(instr[1], instr[2])
        elif "add" in instr:
            self.execute_add(instr[1], instr[2])
        elif "sub" in instr:
            self.execute_sub(instr[1], instr[2])
        elif "mul" in instr:
            self.mult_instr_cnt += 1
            self.execute_mul(instr[1], instr[2])
        elif "mod" in instr:
            self.execute_mod(instr[1], instr[2])
        elif "jgz" in instr:
            if self.get_arg_value(instr[1]) > 0:
                jump_len = self.get_arg_value(instr[2])
        elif "jnz" in instr:
            if self.get_arg_value(instr[1]) != 0:
                jump_len = self.get_arg_value(instr[2])
        elif "snd" in instr:
            self.execute_snd(instr[1])
        elif "rcv" in instr:
            self.execute_rcv(instr[1])

        self.instr_ptr += jump_len

    def execute_sub(self, x, y):
        self.registers[x] -= self.get_arg_value(y)


src = "../res/d23"
input_file = open(src)
instructions = input_file.read().split('\n')
input_file.close()

# PART 1
registers = {}
for i in range(8): registers[chr(ord('a')+i)] = 0
p = CoprocessorProgram(registers, instructions)
p.run()
print(f'(Part1) Number of mult instructions: {p.mult_instr_cnt}')


# PART 2
a = 1
b = 107_900
c = b + 17000
h = 0
# test for each x value from b to c
# stepping 17
while True:
    f = 1
    d = 2

    # test for each d value (from 2)
    # up until constant d = b
    while True:
        e = 2

        # test for each e value
        # up until constant e = b
        while True:
            if d * e - b: f = 0
            e += 1
            if e - b: break 

        d += 1
        if d - b: break

    if f == 0: h += 1

    if b - c: break
    else: b += 17


b = 107_900
c = b + 17000
for i in range(b, c, 17):
    for j in range(2, b):
        for k in range(2, c):
            pass

print(h)