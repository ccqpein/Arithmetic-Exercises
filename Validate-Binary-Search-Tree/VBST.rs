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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let ll = makeAList(&root);
    if ll.len() == 0 {
        return true;
    }
    let mut ll = ll.iter();
    let mut before = ll.next().unwrap();
    for d in ll {
        if *d <= *before {
            return false;
        } else {
            before = d;
        }
    }
    true
}

fn makeAList(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut left: Vec<i32> = vec![];
    if root.as_ref().unwrap().borrow().left.is_some() {
        left = makeAList(&root.as_ref().unwrap().borrow().left);
    }

    let mut right: Vec<i32> = vec![];
    if root.as_ref().unwrap().borrow().right.is_some() {
        right = makeAList(&root.as_ref().unwrap().borrow().right);
    }

    left.push(root.as_ref().unwrap().borrow().val);
    left.append(&mut right);

    left
}
