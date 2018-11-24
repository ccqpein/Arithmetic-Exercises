#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl Clone for ListNode {
    fn clone(&self) -> ListNode {
        let mut result = ListNode::new(self.val);
        result.next = self.next.clone();
        return result;
    }
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
        } else {
            inner_flag = 0;
        }

        match self.next {
            Some(ref mut n) => n.check_ten(inner_flag),
            None => {
                if inner_flag == 1 {
                    self.next = Some(Box::new(ListNode::new(inner_flag)));
                }
            }
        }
    }
}

fn add_two_numbers(a: ListNode, b: ListNode, flag: i32) -> ListNode {
    let mut inner_flag = flag;
    let mut result = ListNode::new(a.val + b.val);

    if a.next.is_some() && b.next.is_some() {
        result.val += inner_flag;
        if result.val >= 10 {
            result.val -= 10;
            inner_flag = 1;
        } else {
            inner_flag = 0;
        }

        let inner_a = Box::leak(a.next.unwrap()).clone();
        let inner_b = Box::leak(b.next.unwrap()).clone();
        result.next = Some(Box::new(add_two_numbers(inner_a, inner_b, inner_flag)))
    } else if a.next.is_none() && b.next.is_some() {
        let temp = Box::leak(b.next.unwrap()).clone();
        result.next = Some(Box::new(temp));
        result.check_ten(inner_flag)
    } else if b.next.is_none() && a.next.is_some() {
        let temp = Box::leak(a.next.unwrap()).clone();
        result.next = Some(Box::new(temp));
        result.check_ten(inner_flag)
    } else if a.next.is_none() && b.next.is_none() {
        result.check_ten(inner_flag);
    }

    return result;
}

fn main() {
    let a = ListNode::new(1);
    let mut b = ListNode::new(9);
    b.next = Some(Box::new(ListNode::new(2)));

    println!("{:?}", add_two_numbers(a, b, 0));

    let c = ListNode::new(2);
    let mut d = ListNode::new(8);
    let mut d1 = ListNode::new(9);
    let mut d2 = ListNode::new(9);
    d1.next = Some(Box::new(d2));
    d.next = Some(Box::new(d1));

    println!("{:?}", add_two_numbers(c, d, 0));
}
