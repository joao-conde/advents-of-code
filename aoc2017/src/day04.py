#Link to problem: https://adventofcode.com/2017/day/4

#For current repos config path is '../res/d04.txt'

src = input("Input file path + extension (e.g.: /dir/file.txt): ")
input_file = open(src)
lines = input_file.readlines()
input_file.close()

#PART1

def validPassPhraseP1(pp):
    words = pp.split(' ')
    
    for word in words:
        if words.count(word) > 1 : return False

    return True


validPPs1 = 0

#PART2

#checks if 2 words are anagrams of each other ; true if so false if not
def isAnagram(word1, word2):
    if len(word1) != len(word2) : return False
        
    for letter in word1:
        if word2.count(letter) == 0 : return False

    return True

def validPassPhraseP2(pp):
    wordsList = pp.split()

    while len(wordsList) > 1:
        head = wordsList[0]
        tail = wordsList[1:]

        for word in tail:
            if(isAnagram(head, word)) : return False
        
        wordsList = tail
    
    return True


validPPs2 = 0

for line in lines:
    line = line.replace('\n', '')
    if validPassPhraseP1(line) : validPPs1 += 1
    if validPassPhraseP2(line) : validPPs2 += 1

#Results print
print("\nP1 valid passphrases: " + str(validPPs1))
print("\nP2 valid passphrases: " + str(validPPs2))





