use std::time::{Duration, Instant};
// Definition for singly-linked list.
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

    #[inline]
    fn new_with_next(val: i32, next: Option<Box<Self>>) -> Self {
        ListNode { next: next, val }
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut all_not_nil = lists
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.clone())
        .collect::<Vec<_>>();
    if all_not_nil.len() == 0 {
        return None;
    }

    let mut smallest: i32 = all_not_nil[0].as_ref().unwrap().val;
    let mut ind: usize = 0;
    for i in 1..all_not_nil.len() as usize {
        if all_not_nil[i].as_ref().unwrap().val <= smallest {
            ind = i;
            smallest = all_not_nil[i].as_ref().unwrap().val;
        }
    }

    let mut result = all_not_nil[ind].as_ref().unwrap().clone();
    all_not_nil[ind] = all_not_nil[ind].as_ref().unwrap().next.clone();
    result.next = merge_k_lists(all_not_nil);
    return Some(result);
}

fn merge_k_lists2(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let now = Instant::now();

    let mut inner_copy_lists = lists.clone();
    let mut result = ListNode::new(0);
    let mut point_to = &mut result;
    let mut length = inner_copy_lists.len().clone();
    while length != 0 {
        let mut ind = 0;
        let mut smallest = inner_copy_lists[0].as_ref().unwrap().val;

        for i in 1..inner_copy_lists.len() {
            let this_v = inner_copy_lists[i].as_ref().unwrap().val;
            if this_v <= smallest {
                smallest = this_v;
                ind = i;
            }
        }

        point_to.next = Some(Box::new(ListNode::new(smallest)));
        point_to = point_to.next.as_mut().unwrap();

        if inner_copy_lists[ind].as_ref().unwrap().next.is_some() {
            inner_copy_lists[ind] = inner_copy_lists[ind].as_ref().unwrap().next.clone();
        } else {
            inner_copy_lists.remove(ind as usize);
            length -= 1;
        }
    }

    let end = now.elapsed().as_nanos();
    println!("{:?} nano seconds", end);
    result.next
}

fn main() {
    use std::thread::sleep;

    let test0 = vec![
        Some(Box::new(ListNode::new_with_next(
            1,
            Some(Box::new(ListNode::new_with_next(
                4,
                Some(Box::new(ListNode::new(5))),
            ))),
        ))),
        Some(Box::new(ListNode::new_with_next(
            1,
            Some(Box::new(ListNode::new_with_next(
                3,
                Some(Box::new(ListNode::new(4))),
            ))),
        ))),
        Some(Box::new(ListNode::new_with_next(
            2,
            Some(Box::new(ListNode::new(6))),
        ))),
    ];

    //let now = Instant::now();
    //merge_k_lists(test0);
    //println!("{:?}", merge_k_lists2(test0));
    merge_k_lists2(test0);
    //sleep(Duration::new(2, 0));
    //let end = now.elapsed().as_nanos();
    //println!("{:?} nano seconds", end);
}
