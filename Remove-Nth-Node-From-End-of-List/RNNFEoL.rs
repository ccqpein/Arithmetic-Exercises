#[derive(Debug)]
struct ListNode {
    Val: i32,
    Next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(i: i32) -> Self {
        ListNode { Val: i, Next: None }
    }

    fn addL(&mut self, l: &Vec<i32>) {
        let org: *mut ListNode = self as *mut ListNode;
        let mut p: *mut ListNode = org;
        for i in l {
            unsafe {
                println!("{:?}", *p);
                (*p).Next = Some(Box::new(ListNode::new(*i)));
                let temp_b = Box::from_raw(p);
                p = Box::into_raw(temp_b.Next.unwrap());
            };
        }
    }
    fn removeNthFromEnd(&self, n: i32) {}
}
fn main() {
    let mut a = ListNode::new(1);
    a.addL(&vec![2, 3, 4, 5]);
    println!("{:?}", a);
}
