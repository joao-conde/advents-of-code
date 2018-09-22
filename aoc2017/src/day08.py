#Link to problem: https://adventofcode.com/2017/day/8

class Register():

    def __init__(self, name):
        self.name = name
        self.cash = 0

    def cash_in(self, cash):
        self.cash += cash

    def cash_out(self, cash):
        self.cash -= cash


#Comparison operations dictionary
ops = {
    ">": (lambda x,y: x > y),
    "<": (lambda x,y: x < y),
    ">=": (lambda x,y: x >= y),
    "<=": (lambda x,y: x <= y),
    "==": (lambda x,y: x == y),
    "!=": (lambda x,y: x != y)
}

#For current repos config path is '../res/d08.txt'
src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
instructions_list= input_file.read().split('\n')
input_file.close()


#PART1 - could do all in one cycle but since the complexity of O(n) is similiar to O(2n) I didn't bother to much since its a small data set

registers = []
max_value_held = "none"

#build registers
for instruction in instructions_list:
    instruction = instruction.split()
    
    target_reg = instruction[0]
    left_operand = instruction[4]
    
    register1 = [reg for reg in registers if reg.name == left_operand]
    register2 = [reg for reg in registers if reg.name == target_reg]
    
    #if it gets and empty list it means there is no register yet with that name so we create one
    if len(register1) == 0 : 
        register1 = Register(left_operand)
        registers.append(register1)
        
    if len(register2) == 0 : 
        register2 = Register(target_reg)
        registers.append(register2)
        

#perform instructions
for instruction in instructions_list:
    instruction = instruction.split()

    left_operand = instruction[4]
    operation = instruction[5]
    right_operand = int(instruction[6])

    reg_cash = [reg.cash for reg in registers if reg.name == left_operand][0]

    if not(ops[operation](reg_cash, right_operand)) :
        continue

    target_reg = [reg for reg in registers if reg.name == instruction[0]][0]
    method = instruction[1]
    cash = int(instruction[2])

    if method == "inc":
        target_reg.cash_in(cash)
    
    if method == "dec":
        target_reg.cash_out(cash)


    #PART2
    if max_value_held == "none" or max_value_held < target_reg.cash : max_value_held = target_reg.cash


#getting max value
cashes = [reg.cash for reg in registers]
print("\nThe largest value in any register after completing the instructions is", max(cashes))
print("Maximum value held during transactions was", max_value_held)





    
        


    
    



