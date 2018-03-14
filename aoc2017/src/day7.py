#Link to problem: https://adventofcode.com/2017/day/7

#Program nodes of calling tree
class Program():

    def __init__(self,name):
        self.name = name
        self.parent = "none"

    def set_parent_prog(self, parent):
        self.parent = parent

    def print_program(self):
        print("My name is", self.name, "and my parent is")


#For current repos config path is '../res/d5input.txt'
src = '../res/d7input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
program_list= input_file.read().split('\n')
input_file.close()



program_nodes = []

for prog in program_list:
    prog = prog.split(' ')
    program_nodes.append(Program(prog[0]))

program_names = [prog.name for prog in program_nodes]

for prog in program_list:
    prog = prog.split(' ')

    if(len(prog) > 2) :
        children = prog[3:]
        children = [child.replace(',','') for child in children]

        for child in children:
            program_nodes[program_names.index(child)].set_parent_prog(prog.name)

root_node = ""
for node in program_nodes:
    print(node.parent)
    if node.parent == "none" : root_node = node
