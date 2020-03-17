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

pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let ss = inner(root);
    //dbg!(&ss);
    return ss.iter().fold(String::new(), |acc, x| {
        lexicographically_less(acc, x.to_string())
    });
}

pub fn inner(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    if root.is_none() {
        return vec![String::new()];
    };

    let mut result = vec![];

    let cell_root = root.unwrap();
    let pure_root = cell_root.borrow();

    match (pure_root.left.clone(), pure_root.right.clone()) {
        (ll @ Some(_), rr @ Some(_)) => {
            result.append(&mut inner(ll));
            result.append(&mut inner(rr));
            return result
                .iter()
                .map(|s| make_string(s.to_string(), pure_root.val))
                .collect();
        }
        (None, rr @ Some(_)) => {
            result.append(&mut inner(rr));
            return result
                .iter()
                .map(|s| make_string(s.to_string(), pure_root.val))
                .collect();
        }
        (ll @ Some(_), None) => {
            result.append(&mut inner(ll));
            return result
                .iter()
                .map(|s| make_string(s.to_string(), pure_root.val))
                .collect();
        }
        (None, None) => {
            return vec![((pure_root.val + 97) as u8 as char).to_string()];
        }
    }
}

// fn lexicographically_less(a: String, b: String, next: i32) -> Vec<String> {
//     let mut aa = a.bytes();
//     let mut bb = b.bytes();

//     loop {
//         let (temp_a, temp_b) = (aa.next(), bb.next());
//         if temp_a.is_none() && temp_b.is_none() {
//             return a;
//         }

//         println!("{:?}, {:?}", temp_a, temp_b);

//         if temp_a.is_none() {
//             if *temp_b.as_ref().unwrap() < ((next + 97) as u8) {
//                 return b;
//             } else if *temp_b.as_ref().unwrap() == ((next + 97) as u8) {
//             } else {
//                 return a;
//             }
//         }
//         if temp_b.is_none() {
//             if *temp_a.as_ref().unwrap() < ((next + 97) as u8) {
//                 return a;
//             } else {
//                 return b;
//             }
//         }
//         if temp_a.unwrap() < temp_b.unwrap() {
//             return a;
//         } else if temp_a.unwrap() > temp_b.unwrap() {
//             return b;
//         }
//     }
// }

fn lexicographically_less(a: String, b: String) -> String {
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }

    let mut aa = a.bytes();
    let mut bb = b.bytes();
    loop {
        let (temp_a, temp_b) = (aa.next(), bb.next());
        if temp_a.is_none() {
            return a;
        };
        if temp_b.is_none() {
            return b;
        };
        if temp_a.unwrap() < temp_b.unwrap() {
            return a;
        } else if temp_a.unwrap() > temp_b.unwrap() {
            return b;
        }
    }
}

fn make_string(a: String, v: i32) -> String {
    let mut aa = a;
    aa.push((v + 97) as u8 as char);
    aa
}

fn main() {
    let a = TreeNode {
        val: 0,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
    };

    dbg!(smallest_from_leaf(Some(Rc::new(RefCell::new(a))))); // dba

    ////////////////
    let b = TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    };

    dbg!(smallest_from_leaf(Some(Rc::new(RefCell::new(b))))); // abc

    //////////////////
    let c = TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
    };

    dbg!(smallest_from_leaf(Some(Rc::new(RefCell::new(c))))); // bae

    ////////////////////
    let d = TreeNode {
        val: 6,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        }))),
    };

    dbg!(smallest_from_leaf(Some(Rc::new(RefCell::new(d))))); // gbg

    /////////////////
    let e = TreeNode {
        val: 25,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
        }))),
        right: None,
    };

    // dbg!(smallest_from_leaf(
    //     "aba".to_string(),
    //     "a".to_string(),
    //     1
    // ));

    dbg!(smallest_from_leaf(Some(Rc::new(RefCell::new(e))))); // ababz
}
