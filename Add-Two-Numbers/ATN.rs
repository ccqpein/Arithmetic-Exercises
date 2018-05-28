use std::rc::Rc;
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(v: i32) -> Self {
        ListNode { val: v, next: None }
    }
}

fn check_ten(this: &mut ListNode, f: &i32) {
    let mut inner_this = this;
    let mut temp: Rc<&mut Box<ListNode>>;
    let mut flag = *f;
    while let Some(t) = { inner_this } {
        t.val += flag;
        if t.val >= 10 {
            t.val = t.val - 10;
            flag = 1;
        } else {
            flag = 0
        }
        temp = Rc::new(t);
        let next = &mut t.next;
        inner_this = next;
    }
    if flag == 1 {
        temp.next = Some(Box::new(ListNode::new(1)));
    }
}

fn main() {
    //let a = ListNode::new(1);
    //let mut b = ListNode::new(9);
    //b.next = Some(Box::new(ListNode::new(2)));

    let mut c = Some(Box::new(ListNode::new(9)));
    check_ten(&mut c, &1);
}
