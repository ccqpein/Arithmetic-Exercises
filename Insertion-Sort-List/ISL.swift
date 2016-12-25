public class ListNode {
    public var val: Int
    public var next: ListNode?
    public init(_ val: Int) {
        self.val = val
        self.next = nil
    }
}

// test cases
var a = ListNode(2)
var b = ListNode(1)
var c = ListNode(3)
a.next = b
c.next = a

// code below
class Solution {
    func insertionSortList(_ head: ListNode?) -> ListNode? {
        var thisNode = head
        var fakehead = head
        guard thisNode != nil else{return nil}
        
        while(thisNode!.next != nil){
            if (thisNode!.next!.val < fakehead!.val){
                (thisNode!.next!.next, thisNode!.next, fakehead) = (fakehead, thisNode!.next!.next, thisNode!.next)
            }else if (thisNode!.next!.val >= fakehead!.val && thisNode!.next!.val < thisNode!.val){
                var tempNode = fakehead
                while(true){
                    if (tempNode!.next!.val > thisNode!.next!.val) {
                        (tempNode!.next, thisNode!.next!.next) = (thisNode!.next, tempNode!.next)
                        break
                    }else{
                        tempNode = tempNode!.next
                    }
                }
            }else{
                thisNode = thisNode!.next
            }
        }
        return fakehead
    }
}


var s = Solution()

let test1 = s.insertionSortList(b) // return b
let test2 = s.insertionSortList(c) // return [1,2,3]
