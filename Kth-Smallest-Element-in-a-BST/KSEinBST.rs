// Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;

fn get_all_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut this_one: Vec<i32> = vec![root.as_ref().unwrap().borrow().val];
    this_one.append(&mut get_all_values(
        root.as_ref().unwrap().borrow().left.clone(),
    ));
    this_one.append(&mut get_all_values(
        root.as_ref().unwrap().borrow().right.clone(),
    ));

    this_one
}

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut ints = get_all_values(root);

    ints.sort();

    ints[(k - 1) as usize]
}

fn main() {
    let testcase0 = Rc::new(RefCell::new(TreeNode::new(1)));

    println!("{:?}", kth_smallest(Some(testcase0), 1));
}
