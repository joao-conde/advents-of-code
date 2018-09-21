#Link to problem: https://adventofcode.com/2017/day/12
     

def find_connections(prog, connections, explored):
    
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



#For current repos config path is '../res/d12input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")

src = '../res/d12input.txt'
input_file = open(src)
pipes = input_file.read().split('\n')
input_file.close()


#PART 1

connections = []

for pipe in pipes:

    #Parse input
    pipe = [x.replace(',', '') for x in pipe.split()]
    pipe = list(filter(lambda a: a != '<->', pipe))
    pipe = [int(x) for x in pipe]

    #List of connections as program sets
    connections.append(set(pipe))

connected_to_0 = 0
for i in range(len(connections)):
    group = find_connections(i, connections, [])
    if 0 in group: connected_to_0 += 1


print("Programs in PROGRAM 0 group:", connected_to_0)