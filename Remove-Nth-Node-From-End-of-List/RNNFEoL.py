# Definition for singly-linked list.
'''
test cases:
[1,2] 2 => [2]
[1,2] 1 => [1]
[1] 1 => []
[1,2,3,4,5] 2 => [1,2,3,5]
'''


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def removeNthFromEnd(self, head, n):
        """
        :type head: ListNode
        :type n: int
        :rtype: ListNode
        """

        firstP = head
        if firstP.next is None:
            return None

        secondP = head
        for i in range(n):
            if secondP.next is not None:
                secondP = secondP.next
            else:
                return head.next

        while(1):
            if firstP.next is not None and secondP.next is not None:
                firstP = firstP.next
                secondP = secondP.next
            else:
                break

            if secondP.next is None:
                break

        if firstP.next is not None and firstP.next.next is not None:
            firstP.next = firstP.next.next
        else:
            firstP.next = None

        return head


a = ListNode(1)
b = ListNode(2)
c = ListNode(3)
d = ListNode(4)
e = ListNode(5)
a.next = b
#b.next = c
c.next = d
d.next = e

so = Solution()
so.removeNthFromEnd(a, 2)

temp = a
while(1):
    print(temp.val)
    if temp.next is None:
        break
    else:
        temp = temp.next
