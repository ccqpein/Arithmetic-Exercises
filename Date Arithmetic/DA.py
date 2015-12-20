#! /usr/bin/env python
# -*- coding:utf-8 -*-

monthDays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def DateArithmetic(date, days):
    dateSeparete = []
    dd = ''
    for i in str(date):
        try:
            ii = int(i)
            dd += i
        except ValueError:
            dateSeparete.append(int(dd))
            continue
    print(dateSeparete, days)
