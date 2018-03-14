#Link to problem: https://adventofcode.com/2017/day/7

#Program nodes of calling tree
class Program():

    def __init__(self,name, weight, disc):
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
        if(self.parent is not None) :
            print("and my parent name is", self.parent.name)


def parse_input(string_list):
    string_list = [x.replace(',',' ') for x in string_list]
    string_list = [x.replace('(',' ') for x in string_list]
    string_list = [x.replace(')',' ') for x in string_list]
    string_list = [x.replace('->',' ') for x in string_list]
    string_list = [" ".join(x.split()) for x in string_list]
    return string_list


#For current repos config path is '../res/d7input.txt'
src = '../res/d7input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
program_list= input_file.read().split('\n')
input_file.close()

program_list = parse_input(program_list)

#creates call tree nodes
program_nodes = []
for program in program_list:
    program = program.split()
    prog_name = program[0]
    prog_weight = program[1]
    prog_disc = []
    if(len(program) > 2) : 
        prog_disc = program[2:]
    program_nodes.append(Program(prog_name, prog_weight, prog_disc))


#associates parents to each node (ancestors)
for node in program_nodes:
    children = [prog for prog in program_nodes if node.has_on_disc(prog.name)]
    
    for child in children:
        child.set_parent_prog(node)
    
#finds root node
root_node = [node for node in program_nodes if node.parent is None][0]
print("The name of the bottom program is:", root_node.name)



