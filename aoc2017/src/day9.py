#Link to problem: https://adventofcode.com/2017/day/9


#counts number of groups inside a group at that level(groups delimited by {...} )
def count_groups(group):
    ngroups = 0
    group_open = False
    
    for char in group:
        if char == '{': group_open = True
        if char == '}' and group_open : 
            group_open = False
            ngroups += 1

    return ngroups

#calculates the score of a group level ---> level score = group level * number of groups
def calculate_score(lvl, group):
    return lvl * count_groups(group)


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
    
print(input_group)

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
print(calculate_score(2,input_group))
