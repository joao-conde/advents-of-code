input_file = open("input/day04")
lines = input_file.readlines()
input_file.close()


# PART1
def valid_pass_p1(pp):
    words = pp.split(" ")
    for word in words:
        if words.count(word) > 1:
            return False
    return True


# PART2
def is_anagram(word1, word2):
    if len(word1) != len(word2):
        return False

    for letter in word1:
        if word2.count(letter) == 0:
            return False

    return True


def valid_pass_p2(pp):
    wordsList = pp.split()

    while len(wordsList) > 1:
        head = wordsList[0]
        tail = wordsList[1:]
        for word in tail:
            if is_anagram(head, word):
                return False
        wordsList = tail

    return True


validPPs1 = 0
validPPs2 = 0
for line in lines:
    line = line.replace("\n", "")
    if valid_pass_p1(line):
        validPPs1 += 1
    if valid_pass_p2(line):
        validPPs2 += 1

print("P1 valid passphrases: " + str(validPPs1))
print("P2 valid passphrases: " + str(validPPs2))
