#! /usr/bin/env python
# -*- coding=utf-8 -*-

def romanToInt(s):
    char1, char2 = 0, 0
    dicOfLetters = {"I": 1, "V": 5, "X": 10, "D": 500, "L": 50, "C": 100, "M": 1000}
    sumNumber = 0
    listOfNumbers = [dicOfLetters[i] for i in s]

    for i in listOfNumbers:
        char2 = i
        sumNumber += char2
        if (char2 == 5 or char2 == 10) and char1 == 1:
            sumNumber -= 2
        elif (char2 == 1000 or char2 == 500) and char1 == 100:
            sumNumber -= 200
        elif (char2 == 100 or char2 ==50) and char1 == 10:
            sumNumber -= 20
        char1 = char2

    return sumNumber
    
