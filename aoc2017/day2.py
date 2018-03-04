#Link to problem: https://adventofcode.com/2017/day/2


input_file = open("day2_input.txt")

#PART1

def getRowDiff(row):
    return max(row) - min(row)

checksum1 = 0

lines = input_file.readlines()

for line in lines:
    line = line.split('\t')
    line = [int(el) for el in line]
    checksum1 = checksum1 + getRowDiff(line)



#PART2

#Returns the division between an element (el) and the first number that evenly divides it in the row, excluding itself
# -1 if none found
def getEvenDiv(row,el):
    for x in row:
        if x == el:
            continue

        if el % x == 0:
            return el / x

    return -1
    
checksum2 = 0

for line in lines:
    line = line.split('\t')
    line = [int(el) for el in line]

    for el in line:
        ret = getEvenDiv(line, el)
        
        if ret != -1:
            checksum2 = checksum2 + ret
            break


print("\nThe checksum1 is " + str(checksum1))
print("\nThe checksum2 is " + str(checksum2))

