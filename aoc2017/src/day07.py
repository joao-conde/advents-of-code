input_file = open("input/day07")
program_list = input_file.read().split("\n")
input_file.close()


class Program:
    def __init__(self, name, weight, disc):
        self.name = name
        self.weight = weight
        self.disc = disc
        self.parent = None

    def has_on_disc(self, prog_name):
        found = True
        try:
            self.disc.index(prog_name)
        except ValueError:
            found = False

        return found

    def set_parent_prog(self, parent):
        self.parent = parent

    def print_program(self):
        print("My name is", self.name, "my disc content is", self.disc)
        if self.parent is not None:
            print("and my parent name is", self.parent.name)


def parse_input(string_list):
    string_list = [x.replace(",", " ") for x in string_list]
    string_list = [x.replace("(", " ") for x in string_list]
    string_list = [x.replace(")", " ") for x in string_list]
    string_list = [x.replace("->", " ") for x in string_list]
    string_list = [" ".join(x.split()) for x in string_list]
    return string_list


def get_disc_weight(program):
    weight = program.weight

    for prog_name in program.disc:
        prog = [x for x in program_nodes if x.name == prog_name][0]
        weight += get_disc_weight(prog)

    return weight


def get_unique_value(values):
    for el in values:
        if values.count(el) == 1:
            return el

    return -1


def get_another_value(element, values):
    for el in values:
        if el != element:
            return el

    return element


program_list = parse_input(program_list)

# PART1

# creates call tree nodes
program_nodes = []
for program in program_list:
    program = program.split()
    prog_name = program[0]
    prog_weight = int(program[1])
    prog_disc = []
    if len(program) > 2:
        prog_disc = program[2:]
    program_nodes.append(Program(prog_name, prog_weight, prog_disc))

# associates parents to each node (ancestors)
for node in program_nodes:
    children = [prog for prog in program_nodes if node.has_on_disc(prog.name)]

    for child in children:
        child.set_parent_prog(node)

root_node = [node for node in program_nodes if node.parent is None][0]
print("The name of the bottom program is:", root_node.name)

# PART2

new_weight = 0
for node in program_nodes:
    children = [child for child in program_nodes if child.name in node.disc]
    children_weights = [get_disc_weight(child) for child in children]

    unique_weight = get_unique_value(children_weights)
    other_value = get_another_value(unique_weight, children_weights)

    if unique_weight != -1:
        unbalanced_node_name = children[children_weights.index(unique_weight)].name
        new_weight = children[children_weights.index(unique_weight)].weight
        diff = unique_weight - other_value
        new_weight -= diff
        break

print(
    "The unbalanced program is",
    unbalanced_node_name,
    "and its new weight is",
    new_weight,
)
