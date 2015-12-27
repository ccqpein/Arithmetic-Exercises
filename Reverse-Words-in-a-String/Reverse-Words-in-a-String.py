def reverseWords(s):
    word, stringR = "", ""
    wordList = []
    i = 0
    
    for c in s:
        i += 1
        if c != " ":
            word += c
           #print(word)
            if i == len(s):
                wordList.append(word)
        elif word != "" and c == " ":
            wordList.append(word)
            word = ""
                
                #print(wordList)
    for i in reversed(wordList):
        stringR += str(i)
        if i != wordList[0]:
            stringR += " "
                
    return stringR
