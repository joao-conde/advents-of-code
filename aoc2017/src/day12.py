#Link to problem: https://adventofcode.com/2017/day/12
     

def find_connections(prog, connections, explored = None):
    
    if explored == None: explored = []

    programs = set()

    for program in connections[prog]:

        #Add known DIRECT CONNECTION
        programs.add(program)

        #If direct connection not explored, do so
        if program not in explored: 

            explored.append(prog)
            indirect_cons = find_connections(program, connections, explored)

            for i_con in indirect_cons:
                programs.add(i_con)

    return programs



#For current repos config path is '../res/d12.txt'
input_file = open(input("Input file path + extension (e.g.: /dir/file.txt): "))
pipes = input_file.read().split('\n')
input_file.close()


#PART 1 & 2

connections = []

for pipe in pipes:

    #Parse input
    pipe = [x.replace(',', '') for x in pipe.split()]
    pipe = list(filter(lambda a: a != '<->', pipe))
    pipe = [int(x) for x in pipe]

    #List of connections as program sets
    connections.append(set(pipe))


connected_to_0 = 0
groups = set()

for i in range(len(connections)):
    
    group = frozenset(find_connections(i, connections)) #frozenset indicates python we do not want to mutate the inner sets
    groups.add(group)

    if 0 in group: connected_to_0 += 1


print("Programs in PROGRAM 0 group:", connected_to_0)
print("Number of groups:", len(groups))