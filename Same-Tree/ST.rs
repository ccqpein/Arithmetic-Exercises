use std::cell::RefCell;
use std::rc::Rc;

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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p.is_none() && q.is_none() {
        true
    } else if p.is_some() && q.is_some() {
        let (a, b) = (p.unwrap(), q.unwrap());
        if a.borrow().val != b.borrow().val {
            false
        } else {
            let aa = Rc::try_unwrap(a).unwrap().into_inner();
            let bb = Rc::try_unwrap(b).unwrap().into_inner();
            is_same_tree(aa.left, bb.left) && is_same_tree(aa.right, bb.right)
        }
    } else {
        false
    }
}

fn main() {}
