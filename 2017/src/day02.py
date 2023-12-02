input_file = open("input/day02")
lines = input_file.readlines()
input_file.close()


# PART1
def get_row_diff(row):
    return max(row) - min(row)


checksum1 = 0
for line in lines:
    line = line.split("\t")
    line = [int(el) for el in line]
    checksum1 = checksum1 + get_row_diff(line)

print("The checksum1 is " + str(checksum1))


# PART2
def get_even_div(row, el):
    for x in row:
        if x == el:
            continue
        if el % x == 0:
            return el // x
    return -1


checksum2 = 0
for line in lines:
    line = line.split("\t")
    line = [int(el) for el in line]

    for el in line:
        ret = get_even_div(line, el)

        if ret != -1:
            checksum2 = checksum2 + ret
            break

print("The checksum2 is " + str(checksum2))
