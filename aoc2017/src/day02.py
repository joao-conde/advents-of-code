#Link to problem: https://adventofcode.com/2017/day/2

#For current repos config path is '../res/d02.txt'

src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)

#PART1

def get_row_diff(row):
    return max(row) - min(row)

checksum1 = 0

lines = input_file.readlines()

input_file.close()

for line in lines:
    line = line.split('\t')
    line = [int(el) for el in line]
    checksum1 = checksum1 + get_row_diff(line)



#PART2

#Returns the division between an element (el) and the first number that evenly divides it in the row, excluding itself
# -1 if none found
def get_even_div(row,el):
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
        ret = get_even_div(line, el)
        
        if ret != -1:
            checksum2 = checksum2 + ret
            break


print("\nThe checksum1 is " + str(checksum1))
print("\nThe checksum2 is " + str(checksum2))

