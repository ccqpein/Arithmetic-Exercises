#! /usr/bin/env python
# -*- coding=utf-8 -*-

def addDigits(num):
    while(1):
        sum= 0
        for i in xrange(len(str(num))):
            sum+= int(str(num)[i])

        if len(str(sum))== 1:
            return sum
        else:
            num= sum
