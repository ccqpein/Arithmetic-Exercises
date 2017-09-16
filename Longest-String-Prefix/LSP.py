def longestPrefix(s):
    templong, stack, longest = 1, s[0], 1
    for c in s[1:]:
        # print(templong, stack, longest)
        if c >= stack:
            templong += 1
        else:
            templong = 0

        if templong > longest:
            longest = templong

        stack = c

    print(longest)

longestPrefix("knotty")
longestPrefix("apple")
longestPrefix("excel")
