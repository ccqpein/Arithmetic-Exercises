#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(v: i32) -> Self {
        ListNode { val: v, next: None }
    }

    fn check_ten(&mut self, flag: i32) {
        let mut inner_flag = flag;
        self.val += flag;
        if self.val >= 10 {
            self.val -= 10;
            inner_flag = 1;
        }
        match self.next {
            Some(ref mut n) => n.check_ten(inner_flag),
            None => if inner_flag == 1 {
                self.next = Some(Box::new(ListNode::new(inner_flag)));
            },
        }
    }
}

fn main() {
    //let a = ListNode::new(1);
    //let mut b = ListNode::new(9);
    //b.next = Some(Box::new(ListNode::new(2)));

    let mut c = Box::new(ListNode::new(9));
    c.check_ten(1);
    println!("{:?}", c);
    println!("{:?}", c.next);
}
