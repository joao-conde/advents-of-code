#Link to problem: https://adventofcode.com/2017/day/4

#For current repos config path is '../res/d4input.txt'

#src = input("Input file path + extension (e.g.: /dir/file.txt): ")
src = '../res/d4input.txt'
input_file = open(src)


#PART1

def validPassPhrase(pp):
    words = pp.split(' ')
    
    for word in words:
        if words.count(word) > 1 : return False

    return True


validPPs = 0

lines = input_file.readlines()


for line in lines:
    line = line.replace('\n', '')
    if validPassPhrase(line) : validPPs += 1


print("\nValid passphrases: " + str(validPPs))


