class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None


def checkTen(n, f):
    flag = f
    temp = n
    while (n is not None):
        n.val += flag
        if n.val >= 10:
            n.val -= 10
            flag = 1
        else:
            flag = 0
        temp = n
        n = n.next

    if flag == 1:
        temp.next = ListNode(1)


class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """

        flag = 0
        result = ListNode(flag)
        temp = result
        last = None

        while(l1 is not None and l2 is not None):
            temp.val += l1.val + l2.val
            if temp.val >= 10:
                temp.val -= 10
                flag = 1
            else:
                flag = 0
            temp.next = ListNode(flag)
            last = temp
            temp = temp.next
            l1 = l1.next
            l2 = l2.next

        if l1 is None:
            if l2 is not None:
                last.next = l2
                checkTen(l2, flag)
            else:
                if flag == 1:
                    last.next = ListNode(flag)
                else:
                    last.next = None
            return result

        if l2 is None:
            last.next = l1
            checkTen(l1, flag)

        return result


'''
test:
(1)
(9,2)
-> (0,3)

(2)
(8,9,9)
->(0,0,0,1)

(5)
(5)
->(0,1)

'''
