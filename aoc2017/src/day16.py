input_file = open("input/day16")
instructions = input_file.read().split(",")
instructions = [str(instruction) for instruction in instructions]
input_file.close()


def spin(programs, size):
    return programs[-size:] + programs[:-size]


def exchange(programs, idx1, idx2):
    temp = programs[idx1]
    programs[idx1] = programs[idx2]
    programs[idx2] = temp
    return programs


def partner(programs, p1, p2):
    idx1, idx2 = programs.index(p1), programs.index(p2)
    programs[idx1], programs[idx2] = p2, p1
    return programs


# PART 1 & 2
iterations = 1_000_000_000
configs = []
programs = [chr(ord("a") + i) for i in range(16)]

for i in range(iterations):
    for instruction in instructions:
        if instruction[0] == "s":
            programs = spin(programs, int(instruction[1:]))

        elif instruction[0] == "x":
            args = instruction.replace("x", "").split("/")
            programs = exchange(programs, int(args[0]), int(args[1]))

        elif instruction[0] == "p":
            args = instruction[1:].split("/")
            programs = partner(programs, args[0], args[1])

    if "".join(programs) in configs:
        break
    else:
        configs.append("".join(programs))

print("Programs order after dance:", "".join(programs))
print(
    "Programs order after their billion dances:",
    configs[(iterations % len(configs)) - 1],
)
