#! /usr/bin/env python
# -*- coding=utf-8 -*-

def moveZeroes(nums):
    x= 0
    times= len(nums)
    for i in xrange(times):
        if nums[x]== 0:
            nums= nums[:x]+ nums[x+1:len(nums)]+ [0]
        else:
            x+= 1
    print(nums)
