#Link to problem: https://adventofcode.com/2017/day/9


#get subgroups
def get_subgroups(group):
    subgroups = []

    i = 1
    while i < len(group): 
        
        if group[i] == '{' and group[i+1] != '{':
            j = i
            while j < len(group):
                if group[j] == '}' :
                    print(group[i:j+1])
                    break
                j += 1

        i += 1

    return subgroups


#For current repos config path is '../res/d9input.txt'
src = "../res/d9input.txt"
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
input_group = input_file.read()
input_file.close()
print(input_group)


subgroups = get_subgroups(input_group)
for subgroup in subgroups:
    print(subgroup)
    subgroups.append(get_subgroups(subgroup))


#PART1 

#clean group string: remove ! and the char they negate
i = 0
while i < len(input_group):

    if input_group[i] == '!' : 
        input_group = input_group[:i] + input_group[(i+1):]
        input_group = input_group[:i] + input_group[(i+1):] 

    i += 1
    

#clean the garbage - in between '<...>'
i = 0
garbage = False
while i < len(input_group):

    if input_group[i] == '>' :
        input_group = input_group[:i] + input_group[(i+1):]
        garbage = False
        i -= 1

    if input_group[i] == '<' :
        garbage = True 

    if garbage : 
        input_group = input_group[:i] + input_group[(i+1):]
        i -= 1

    i += 1
    
