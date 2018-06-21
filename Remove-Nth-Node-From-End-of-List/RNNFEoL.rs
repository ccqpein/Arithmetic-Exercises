#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(i: i32) -> Self {
        ListNode { val: i, next: None }
    }

    //:= DEBUG: self point changed
    //:= https://stackoverflow.com/questions/50957738/how-to-copy-a-raw-pointer-in-rust?noredirect=1#comment88913158_50957738
    fn add_l(&mut self, l: &Vec<i32>) {
        //let mut p: *mut ListNode = self as *mut ListNode;
        let org = self;
        let mut p: *mut ListNode = org as *mut ListNode;

        for i in l {
            unsafe {
                //println!("{:?}", *p);
                (*p).next = Some(Box::new(ListNode::new(*i)));
                let temp_b = Box::from_raw(p);
                p = Box::into_raw(temp_b.next.unwrap());
            };
        }
    }

    fn add(&mut self, i: &i32) {
        self.next = Some(Box::new(ListNode::new(*i)));
    }

    fn add_head(self, i: i32) -> Self {
        ListNode {
            val: i,
            next: Some(Box::new(self)),
        }
    }

    fn new_l(l: Vec<i32>) -> Self {
        let mut inner_l = l;
        let mut temp = Self::new(*&inner_l.pop().unwrap());
        for i in inner_l.into_iter().rev() {
            temp = temp.add_head(i)
        }
        temp
    }

    fn removeNthFromEnd(&self, n: i32) {}
}
fn main() {
    //let mut a = ListNode::new(1);
    //a.add_l(&vec![2, 3, 4, 5]);

    let a = ListNode::new_l(vec![1, 2, 3, 4, 5]);
    println!("{:?}", a);
}
