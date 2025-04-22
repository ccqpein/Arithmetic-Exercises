use std::cell::RefCell;
use std::collections::VecDeque;
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

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&mut preorder.into(), Some(inorder))
}

fn helper(
    preorder: &mut VecDeque<i32>,
    inorder: Option<Vec<i32>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    dbg!(&preorder);
    dbg!(&inorder);

    if inorder.is_none() || inorder.as_ref().unwrap().len() == 0 {
        return None;
    }

    let root = preorder.pop_front().unwrap();

    let (left, right) = split(inorder.unwrap(), root);
    dbg!(&left);
    dbg!(&right);

    let ll = helper(preorder, left);
    let rr = helper(preorder, right);

    let mut tt = TreeNode::new(root);
    tt.left = ll;
    tt.right = rr;
    Some(Rc::new(RefCell::new(tt)))
}

fn split(inorder: Vec<i32>, k: i32) -> (Option<Vec<i32>>, Option<Vec<i32>>) {
    let mut ss = inorder.split(|v| *v == k);
    let left = ss.next().map(|v| v.to_vec());
    let right = ss.next().map(|v| v.to_vec());

    (left, right)
}

fn main() {
    //dbg!(split(vec![9, 3, 15, 20, 7], 3));

    dbg!(helper(
        &mut vec![3, 9, 20, 15, 7].into(),
        Some(vec![9, 3, 15, 20, 7])
    ));
}
