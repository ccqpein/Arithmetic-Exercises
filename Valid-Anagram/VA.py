def isAnagram(s, t):
    t = reversed(t)
    if t == "":
        return False
    time = 0
    for i in t:
        if s[time] == i:
            time += 1
            continue
        else:
            return False
    return True
