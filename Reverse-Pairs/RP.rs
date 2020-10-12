// of course this method is too slow
// pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
//     if nums.len() < 2 {
//         return 0;
//     }

//     let nums: Vec<i64> = nums.iter().map(|&s| s as i64).collect();

//     let mut count = 0;
//     for i in 0..nums.len() {
//         for j in 0..i {
//             if nums[j] > nums[i] * 2 {
//                 count += 1
//             }
//         }
//     }
//     count
// }

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
struct BST {
    this: i64,
    count: usize, // count number of larger or equal number in this tree
    left: Option<Rc<RefCell<BST>>>,

    right: Option<Rc<RefCell<BST>>>,
}

impl BST {
    fn new(n: i64) -> Self {
        BST {
            this: n,
            count: 1,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, n: i64) {
        if n > self.this {
            self.count += 1;
            if let Some(t) = self.right.as_ref() {
                t.borrow_mut().insert(n);
            } else {
                self.right = Some(Rc::new(RefCell::new(BST::new(n))))
            }
        } else if n < self.this {
            if let Some(t) = self.left.as_ref() {
                t.borrow_mut().insert(n);
            } else {
                self.left = Some(Rc::new(RefCell::new(BST::new(n))))
            }
        } else {
            self.count += 1;
        }
    }

    fn search(&self, n: i64) -> usize {
        if self.this == n {
            self.count
        } else if n < self.this {
            if let None = self.left {
                self.count
            } else {
                self.count + self.left.as_ref().unwrap().borrow().search(n)
            }
        } else {
            if let None = self.right {
                0
            } else {
                self.right.as_ref().unwrap().borrow().search(n)
            }
        }
    }
}

// this method is O(n^2)
pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let mut nums = nums.iter().map(|&s| s as i64);

    let mut count = 0;
    let mut bst = BST::new(nums.next().unwrap());

    while let Some(n) = nums.next() {
        count += bst.search(2 * n + 1);
        bst.insert(n);
    }

    count as i32
}

fn main() {
    // assert_eq!(2, reverse_pairs(vec![1, 3, 2, 3, 1]));
    // assert_eq!(3, reverse_pairs(vec![2, 4, 3, 5, 1]));
    // assert_eq!(0, reverse_pairs(vec![1]));
    // assert_eq!(4, reverse_pairs(vec![5, 4, 3, 2, 1]));
    // assert_eq!(
    //     0,
    //     reverse_pairs(vec![
    //         2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
    //     ])
    // );
    assert_eq!(
        9,
        reverse_pairs(vec![
            2147483647,
            2147483647,
            -2147483647,
            -2147483647,
            -2147483647,
            2147483647
        ])
    );

    assert_eq!(
        40,
        reverse_pairs(vec![
            233, 2000000001, 234, 2000000006, 235, 2000000003, 236, 2000000007, 237, 2000000002,
            2000000005, 233, 233, 233, 233, 233, 2000000004
        ])
    );
}
