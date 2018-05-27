class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None
        
class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        result = ListNode(l1.val + l2.val)
        flag = 0
        if l1.val + l2.val >= 10:
            result.val = l1.val + l2.val - 10
            flag = 1
            
        temp = ListNode(flag)
        result.next = temp
        last=result
        
        while(l1.next is not None and l2.next is not None):
            l1 = l1.next
            l2 = l2.next
            temp.val += l1.val + l2.val
            if l1.val + l2.val >= 10:
                temp.val = l1.val + l2.val - 10
                flag = 1
            else:
                flag = 0
                
            temp.next = ListNode(flag)
            last = temp
            temp = temp.next

        if l1.next is None:
            if l2.next is not None:
                if flag == 1:
                    l2.next.val += 1
                    if l2.next.val >= 10:
                        l2.next.val -= 10
                        l2.next.next = ListNode(1)
                last.next = l2.next
            else:
                if flag == 1:
                    temp = ListNode(1)
                else:
                    last.next = None
                return result

        if l2.next is None:
            if flag == 1:
                l1.next.val += 1
                if l1.next.val >= 10:
                        l1.next.val -= 10
                        l1.next.next = ListNode(1)
                last.next = l1.next
            else:
                print("here")
                last.next = l1.next

        return result

'''
a = ListNode(2)
a.next = ListNode(4)
a.next.next = ListNode(3)

b = ListNode(5)
b.next = ListNode(6)
b.next.next = ListNode(4)
'''

a = ListNode(1)
a.next = ListNode(8)

b = ListNode(0)

s = Solution()
re= s.addTwoNumbers(a,b)
