use std::cell::RefCell;
use std::rc::Rc;

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

// return number of nodes and sum of numbers
fn helper(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> (f32, f32) {
    if root.is_none() {
        return (0_f32, 0_f32);
    }

    let sum = root.as_ref().unwrap().borrow().val as f32;
    let left_v = helper(&root.as_ref().unwrap().borrow().left, count);
    let right_v = helper(&root.as_ref().unwrap().borrow().right, count);

    // dbg!(sum + left_v.1 + right_v.1);
    // dbg!((1.0 + left_v.0 + right_v.0));
    if ((sum + left_v.1 + right_v.1) / (1.0 + left_v.0 + right_v.0)).floor() == sum {
        //dbg!(root);
        *count += 1;
    }

    (1_f32 + left_v.0 + right_v.0, sum + left_v.1 + right_v.1)
}

pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut count = 0;
    helper(&root, &mut count);
    count
}

fn main() {
    let case0 = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        }))),
    })));

    let mut count = 0;
    helper(&case0, &mut count);
    dbg!(count);

    let case1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        }))),
    })));

    let mut count = 0;
    helper(&case1, &mut count);
    dbg!(count);
}
