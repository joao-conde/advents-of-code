#Link to problem: https://adventofcode.com/2017/day/9


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
