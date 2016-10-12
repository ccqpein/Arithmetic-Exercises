#! /usr/bin/env Python3

import math


def Entropy(l):  # l is the list included times of each variable
    sumNum = sum(l)
    result = 0
    for i in l:
        if i == 0:
            result += 0
        else:
            result += - (i / sumNum) * math.log2(i / sumNum)

    return result


class Node():

    def __init__(self, data):
        self.this = data
        self.childrenNode = {}

    def addNode(self, childNode):  # might be changed futrue

        # the key of dict might be changed in futrue
        self.childrenNode[childNode.this] = childNode


if __name__ == "__main__":
    print("")
