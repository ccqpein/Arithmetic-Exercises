use std::collections::HashSet;

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

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let ss: HashSet<i32> = nums.into_iter().collect();
    helper(&ss, head)
}

fn helper_bkp(set: &HashSet<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let v: i32 = head.as_ref().unwrap().as_ref().val;
    if set.contains(&v) {
        helper(set, head.unwrap().as_mut().next.clone())
    } else {
        let nn = head.as_ref().unwrap().as_ref().next.clone();
        head.as_mut().unwrap().as_mut().next = helper(set, nn);
        head.clone()
    }
}

fn helper(set: &HashSet<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_head = head;
    while let Some(ref mut node) = new_head {
        if set.contains(&node.val) {
            new_head = node.next.take();
        } else {
            break;
        }
    }

    let mut curr = new_head.as_mut().unwrap();

    while let Some(node) = curr.next.as_mut() {
        if set.contains(&node.val) {
            curr.next = node.next.take();
        } else {
            curr = curr.next.as_mut().unwrap();
        }
    }

    new_head
}

fn main() {}
