use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut re = vec![];
    if root.is_none() {
        return re;
    }

    let mut next = vec![root.unwrap()];
    let mut vv: i32;

    loop {
        (next, vv) = helper(&next);
        if next.len() == 0 {
            re.push(vv);
            break;
        }

        re.push(vv);
    }
    re
}

fn helper(trees: &[Rc<RefCell<TreeNode>>]) -> (Vec<Rc<RefCell<TreeNode>>>, i32) {
    let mut max = i32::MIN;
    let mut next_round = vec![];

    for v in trees {
        if v.borrow().val > max {
            max = v.borrow().val.clone()
        }

        match (&v.borrow().left, &v.borrow().right) {
            (None, None) => (),
            (None, Some(r)) => next_round.push(r.clone()),
            (Some(l), None) => next_round.push(l.clone()),
            (Some(l), Some(r)) => {
                next_round.push(r.clone());
                next_round.push(l.clone());
            }
        }
    }

    (next_round, max)
}

fn main() {}
