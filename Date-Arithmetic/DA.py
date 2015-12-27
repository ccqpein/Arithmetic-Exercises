#! /usr/bin/env python
# -*- coding:utf-8 -*-

monthDays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def DateArithmetic(date, days):
    dateSeparete = []
    dd = ''
    for i in date:
        try:
            ii = int(i)
            dd += i
        except ValueError:
            dateSeparete.append(int(dd))
            dd = ''
            continue
    dateSeparete.append(int(dd))

    while(days):
        if dateSeparete[2] % 4 == 0:
            monthDays[1] = 29
        else:
            monthDays[1] = 28
        if days > (monthDays[dateSeparete[0] - 1] - dateSeparete[1]):
            days -= monthDays[dateSeparete[0] - 1] - dateSeparete[1]
            dateSeparete[0] += 1
            dateSeparete[1] = 0
            if dateSeparete[0] == 13:
                dateSeparete[0] = 1
                dateSeparete[2] += 1
            continue
        else:
            dateSeparete[1] += days
            days = 0

    print(dateSeparete)
