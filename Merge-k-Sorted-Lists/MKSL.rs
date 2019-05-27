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

fn main() {}
