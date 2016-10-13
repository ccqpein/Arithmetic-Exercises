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


class Leaf():

    def __init__(self, **kwargs):
        self.leaf = kwargs


class Node():

    def __init__(self, tag):
        self.this = tag
        self.childList = []
        self.childNode = {}
        self.leaves = {}

    def addNode(self, chileWay, childNode):  # might be changed futrue
        self.childNode[chileWay] = childNode
        self.childList.append(chileWay)

    def addLeaf(self, chileWay, leaf):
        self.leaves[chileWay] = leaf
        self.childList.append(chileWay)

    def search(self, *attributions):
        pass


def main():

    a = Node("outlook?")
    b1 = Node("humidity?")
    b2 = Leaf("play": 4, "noPlay": 0)
    b3 = Node("windy?")

    c1 = Leaf("play": 2, "noPlay": 0)
    c2 = Leaf("play": 0, "noPlay": 3)
    c3 = Leaf("play": 0, "noPlay": 2)
    c4 = Leaf("play": 3, "noPlay": 0)

    a.addNode("sunny", b1)
    a.addNode("rain", b3)
    a.addLeaf("overcast", b2)

    b1.addLeaf("<=70", c1)
    b1.addLeaf(">70", c2)

    b3.addLeaf("true", c3)
    b3.addLeaf("false", c4)

    print(c1[play])


if __name__ == "__main__":
    print("")
