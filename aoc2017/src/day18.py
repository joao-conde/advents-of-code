#Link to problem: https://adventofcode.com/2017/day/18

class Program(object):
    def __init__(self, registers, instructions = [], instr_ptr = 0):
        self.registers = registers
        self.instructions = instructions
        self.instr_ptr = instr_ptr

    def run(self):
        while self.instr_ptr >= 0 and self.instr_ptr < len(self.instructions):
            self.execute_instr(instructions[self.instr_ptr].split())

    def get_arg_value(self, a):
        try:
            value = int(a)
        except ValueError:
            value = self.registers[a]
        return value

    def execute_instr(self, instr):
        jump_len = 1

        if "set" in instr:
            self.execute_set(instr[1], instr[2])
        elif "add" in instr:
            self.execute_add(instr[1], instr[2])
        elif "mul" in instr:
            self.execute_mul(instr[1], instr[2])
        elif "mod" in instr:
            self.execute_mod(instr[1], instr[2])
        elif "jgz" in instr:
            if self.get_arg_value(instr[1]) > 0:
                jump_len = self.get_arg_value(instr[2])
        elif "snd" in instr:
            self.execute_snd(instr[1])
        elif "rcv" in instr:
            self.execute_rcv(instr[1])

        self.instr_ptr += jump_len

    def execute_set(self, x, y): 
        self.registers[x] = self.get_arg_value(y)
    
    def execute_add(self, x, y): 
        self.registers[x] += self.get_arg_value(y)

    def execute_mul(self, x, y): 
        self.registers[x] *= self.get_arg_value(y)
    
    def execute_mod(self, x, y): 
        self.registers[x] %= self.get_arg_value(y)

    def execute_snd(self, x): pass

    def execute_rcv(self, x): pass

class SoundProgram(Program):
    def __init__(self, registers, instructions = [], instr_ptr = 0):
        super().__init__(registers, instructions, instr_ptr)
        self.last_played_frequency = None
    
    def run(self):
        while self.instr_ptr >= 0 and self.instr_ptr < len(self.instructions):
            instr = instructions[self.instr_ptr].split()
            self.execute_instr(instr)
            if "rcv" in instr: return self.last_played_frequency
            
    def execute_snd(self, x):
        self.last_played_frequency = self.get_arg_value(x)

    def execute_rcv(self, x):
        if self.get_arg_value(x) != 0:
            return self.last_played_frequency

src = "../res/d18"

input_file = open(src)
instructions = input_file.read().split('\n')
input_file.close()

# PART 1 
registers = {}
for i in range(26): registers[chr(ord('a')+i)] = 0
p = SoundProgram(registers, instructions)
print(f'(Part1) First recovered frequency: {p.run()}')


# PART 2

# # p1_queue.put(4)
# # while not p1_queue.empty():
# #     print(p1_queue.get())
