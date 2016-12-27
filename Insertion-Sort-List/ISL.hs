data ListNode a = Empty | ListNode a (ListNode a) deriving (Show, Read)
-- actually, make a new data type is not necessary, so I comment ths code below

{--
b = ListNode 1 Empty
a = ListNode 2 b
c = ListNode 3 a
--}

insertionSList :: [Int] -> [Int] -> [Int]
insertionSList a [] = a
insertionSList a (x:xs) = insertionSList (insertList a x) xs where
  insertList [] a = a:[]
  insertList (x:xs) a
    | a < x = a:x:xs
    | a >= x = x:(insertList xs a)

-- test: insertionSList [] [2,3,1]
