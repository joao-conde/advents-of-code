#Link to problem: https://adventofcode.com/2017/day/7


class Program():

    def __init__(self,name,weight,disc):
        self.name = name
        self.weight = weight
        self.disc = disc
        self.caller = "none"

    def setCaller(self, caller):
        self.caller = caller

    def isOnDisc(self, prog):
        return prog in disc


    def printProgram(self):
        print("My name is", self.name, "I weight", self.weight, "and my disc content is", disc)


#For current repos config path is '../res/d5input.txt'
src = '../res/d7input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
programList= input_file.read().split('\n')
input_file.close()


#parse of input and creation of Program objects
programs = []
for program in programList:

    programParams = program.split(' ')

    disc = []   
    if len(programParams) > 2 : disc = programParams[3:]

    prog = Program(programParams[0], programParams[1][1:][:-1], disc)
    programs.append(prog)
    prog.printProgram()



for prog1 in programs:
    for prog2 in programs:
        #print(prog1.name, " + ", prog2.name)
        if(prog1.isOnDisc(prog2.name)) : 
            #print(prog2.name, "is on disc of", prog1.name)
            prog2.setCaller(prog1.name)



for program in programs:
    if(program.caller == "none") : print(program.name)




