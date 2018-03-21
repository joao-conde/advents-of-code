#Link to problem: https://adventofcode.com/2017/day/9

#get subgroups
def get_subgroups(group):
    subgroups = []

    i = 1
    while i < len(group): 
        
        if group[i] == '{':
            nest_lvl = 0
            offset = 1
            next_char = group[i+offset]
            while next_char == '{':
                nest_lvl += 1
                offset += 1
                next_char = group[i+offset]


            j = i
            while j < len(group):
                if group[j] == ',' : nest_lvl += 1
                if group[j] == '}' :
                    if nest_lvl == 0:
                        #print(group[i:j+1])
                        subgroups.append(group[i:j+1])
                        i = j
                        break
                    else:
                        nest_lvl -= 1
                j += 1

        i += 1

    return subgroups


#For current repos config path is '../res/d9input.txt'
src = "../res/d9input.txt"
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
input_group = input_file.read()
input_file.close()



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
    


print(input_group)
print(get_subgroups(input_group)[0].split(','))
