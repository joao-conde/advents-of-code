from day18 import Program

input_file = open("input/day23")
instructions = input_file.read().split("\n")
input_file.close()


class CoprocessorProgram(Program):
    def __init__(self, registers, instructions=[], instr_ptr=0):
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


# PART 1
registers = {}
for i in range(8):
    registers[chr(ord("a") + i)] = 0
p = CoprocessorProgram(registers, instructions)
p.run()
print(f"Number of mult instructions: {p.mult_instr_cnt}")

# PART 2
b = 107_900
c = b + 17_000
non_prime = 0
for i in range(b, c + 1, 17):
    for j in range(2, i):
        if i % j == 0:
            non_prime += 1
            break

print(f"Register 'h' contains how many non-primes from {b} to {c} exist: {non_prime}")
