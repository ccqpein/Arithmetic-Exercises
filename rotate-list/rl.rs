#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    let len = len_list_node(&head);
    if len == 0 {
        return None;
    }

    k = k % len;
    if k == 0 {
        return head;
    }

    let mut this = &head;
    let mut prev = vec![];
    let mut tail = vec![];

    for _ in 0..(len - k) {
        prev.push(this.as_ref().unwrap().val);
        this = &this.as_ref().unwrap().next;
    }

    loop {
        if this.is_none() {
            break;
        }
        tail.push(this.as_ref().unwrap().val);
        this = &this.as_ref().unwrap().next;
    }

    tail.append(&mut prev);
    make_list_node(tail)
}

fn len_list_node(mut head: &Option<Box<ListNode>>) -> i32 {
    let mut len = 0;
    let mut prev = &None;
    loop {
        if head.is_none() {
            return len;
        }

        len += 1;
        prev = head;
        head = &head.as_ref().unwrap().next;
    }
}

fn make_list_node(mut vals: Vec<i32>) -> Option<Box<ListNode>> {
    vals.reverse();
    let mut a = None;
    for v in vals {
        a = Some(Box::new(ListNode { val: v, next: a }));
    }

    a
}

fn main() {
    let a = make_list_node(vec![1, 2, 3, 4, 5]);
    //dbg!(&a);
    dbg!(rotate_right(a, 2));
}
