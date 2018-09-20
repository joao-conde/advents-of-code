#Link to problem: https://adventofcode.com/2017/day/12
     

class Node:

    def __init__(self, id):
        self.id = id
        self.parents = set() #incoming nodes
                
    def add_parent_node(self, node):
        self.parents.add(node)


def are_connected(n1, n2, visited):

    if n1 in n2.parents: return True

    if n2 not in visited: 
        visited.add(n2)

    for node in n2.parents:
        return are_connected(n1, node, visited)

#For current repos config path is '../res/d12input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")

src = '../res/d12input.txt'
input_file = open(src)
pipes = input_file.read().split('\n')
input_file.close()


nodes = [Node(x) for x in range(len(pipes))]

for pipe in pipes:

    pipe = pipe.split()
    pipe = [x.replace(',', '').replace('<->', '') for x in pipe]
    pipe = list(filter(lambda a: a != '', pipe))

    for i in range(len(pipe)):
        nodes[int(pipe[0])].add_parent_node(nodes[int(pipe[i])])


# for node in nodes:
#     print("\nNODE",node.id, end=' ')
#     print("PARENTS:", end=' ')
#     for p in node.parents: print(p.id, end=' ')


print(are_connected(nodes[0], nodes[3], set()))