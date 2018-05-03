# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution:
    def middleNode(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        temp0 = head
        if head.next is None:
            return temp0
        temp1 = head.next

        while True:
            if temp1.next is None or temp1.next.next is None:
                return temp0.next

            temp0 = temp0.next
            temp1 = temp1.next.next
