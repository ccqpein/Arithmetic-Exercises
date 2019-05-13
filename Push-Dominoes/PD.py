def make_dot(count, left, right):
    if left == "" and right == "":
        return count * "."

    if right == "":
        if left == "L":
            return '.'*count
        else:
            return 'R'*count

    if left == "":
        if right == "L":
            return count * "L"
        else:
            return count * "."

    if left == "L" and right == "R":
        return '.'*count
    elif right == "L" and left == "R":
        if count % 2 == 1:
            return ((count//2) * "R") + "." + ((count//2) * "L")
        else:
            return (count//2) * 'R' + 'L' * (count//2)
    elif left == "L" and right == "L":
        return count*'L'
    elif left == "R" and right == "R":
        return count*'R'


class Solution:
    def pushDominoes(self, dominoes):
        result_list = []
        letter_cache = []
        last = ''
        dot_count = 0
        for i in dominoes:
            if i == ".":
                if last != '' and last != '.':
                    result_list.append("".join(letter_cache))
                    letter_cache = []
                dot_count += 1
            else:
                if last == ".":
                    result_list.append(dot_count)
                    letter_cache.append(i)
                    dot_count = 0
                else:
                    letter_cache.append(i)
            last = i

        if dot_count != 0:
            result_list.append(dot_count)
        elif letter_cache != []:
            result_list.append("".join(letter_cache))

        print(result_list)

        for ind in range(len(result_list)):
            left = ''
            right = ''
            if type(result_list[ind]) == int:
                if ind-1 >= 0:
                    left = result_list[ind-1][-1]
                if ind+1 < len(result_list):
                    right = result_list[ind+1][0]
                result_list[ind] = make_dot(result_list[ind], left, right)

        return "".join(result_list)


a = Solution()
a.pushDominoes(".L.R...LR..L..") == "LL.RR.LLRRLL.."
a.pushDominoes("RR.L") == "RR.L"
a.pushDominoes(".") == "."
a.pushDominoes("LL") == "LL"
