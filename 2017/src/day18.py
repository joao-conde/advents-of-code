class Program(object):
    def __init__(self, registers, instructions=[], instr_ptr=0):
        self.registers = registers
        self.instructions = instructions
        self.instr_ptr = instr_ptr

    def run(self):
        while not self.done():
            self.execute_instr(self.instructions[self.instr_ptr].split())

    def done(self):
        return self.instr_ptr < 0 or self.instr_ptr >= len(self.instructions)

    def get_arg_value(self, val):
        return self.registers[val] if val.isalpha() else int(val)

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

    def execute_snd(self, x):
        pass

    def execute_rcv(self, x):
        pass


class SoundProgram(Program):
    def __init__(self, registers, instructions=[], instr_ptr=0):
        super().__init__(registers, instructions, instr_ptr)
        self.last_played_frequency = None

    def run(self):
        while not self.done():
            instr = instructions[self.instr_ptr].split()
            self.execute_instr(instr)
            if "rcv" in instr:
                return self.last_played_frequency

    def execute_snd(self, x):
        self.last_played_frequency = self.get_arg_value(x)

    def execute_rcv(self, x):
        if self.get_arg_value(x) != 0:
            return self.last_played_frequency


class MessageProgram(Program):
    def __init__(self, registers, instructions=[], instr_ptr=0):
        super().__init__(registers, instructions, instr_ptr)
        self.msg_queue = []
        self.other_msg_program = None
        self.sent_msgs = 0
        self.awaiting_msg = False

    def execute_snd(self, x):
        self.other_msg_program.msg_queue.append(self.get_arg_value(x))
        self.sent_msgs += 1

    def execute_rcv(self, x):
        if len(self.msg_queue) > 0:
            self.awaiting_msg = False
            self.registers[x] = self.msg_queue[0]
            self.msg_queue = self.msg_queue[1:]
        else:
            self.instr_ptr -= 1  # prevent point advance
            self.awaiting_msg = True


if __name__ == "__main__":
    input_file = open("input/day18")
    instructions = input_file.read().split("\n")
    input_file.close()

    # PART 1
    registers = {}
    for i in range(26):
        registers[chr(ord("a") + i)] = 0
    p = SoundProgram(registers, instructions)
    print(f"First recovered frequency: {p.run()}")

    # PART 2
    p0_registers, p1_registers = {}, {}
    for i in range(26):
        p0_registers[chr(ord("a") + i)] = 0
        p1_registers[chr(ord("a") + i)] = 0
    p1_registers["p"] = 1

    p0, p1 = MessageProgram(p0_registers, instructions), MessageProgram(
        p1_registers, instructions
    )
    p0.other_msg_program = p1
    p1.other_msg_program = p0

    while not (
        p0.awaiting_msg
        and len(p0.msg_queue) == 0
        and p1.awaiting_msg
        and len(p1.msg_queue) == 0
    ):
        instr = instructions[p0.instr_ptr].split()
        p0.execute_instr(instr)

        instr = instructions[p1.instr_ptr].split()
        p1.execute_instr(instr)

    print(f"Messages sent by program 1: {p1.sent_msgs}")
