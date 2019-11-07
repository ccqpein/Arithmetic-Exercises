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

pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_UT_WithState(root.as_ref(), root.as_ref().unwrap().borrow().val.clone())
}

pub fn is_UT_WithState(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> bool {
    if root.is_none() {
        return true;
    }
    let this_root = root.as_ref().unwrap();
    let this_val = this_root.borrow().val;
    if this_val != val {
        return false;
    }

    is_UT_WithState(this_root.borrow().left.as_ref(), this_val)
        && is_UT_WithState(this_root.borrow().right.as_ref(), this_val)
}
