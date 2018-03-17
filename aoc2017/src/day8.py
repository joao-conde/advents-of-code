#Link to problem: https://adventofcode.com/2017/day/8


#For current repos config path is '../res/d8input.txt'
src = '../res/d8input.txt'
#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
program_list= input_file.read().split('\n')
input_file.close()
