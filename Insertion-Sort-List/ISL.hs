data ListNode a = Empty | ListNode a (ListNode a) deriving (Show, Read)

b = ListNode 1 Empty
a = ListNode 2 b
c = ListNode 3 a

