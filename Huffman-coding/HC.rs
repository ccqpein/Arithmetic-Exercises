use std::collections::VecDeque;
use std::rc::Rc;
use std::{borrow::Borrow, cmp::Ordering};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, Default)]
struct BTree {
    c: Option<char>,
    value: usize,
    left: Option<Rc<RefCell<BTree>>>,
    right: Option<Rc<RefCell<BTree>>>,
}

impl BTree {
    fn new(c: char, v: usize) -> Self {
        Self {
            c: Some(c),
            value: v,
            left: None,
            right: None,
        }
    }

    fn new_from_two_self_rc(a: PBTree, b: PBTree) -> Self {
        let v = a.as_ref().borrow().value + b.as_ref().borrow().value;
        Self {
            c: None,
            value: v,
            left: Some(a),
            right: Some(b),
        }
    }

    fn new_from_code(&mut self, c: char, mut code: Vec<char>) -> Result<(), usize> {
        if code.len() == 1 {
            match code[0] {
                '0' => {
                    if self.left.is_some() {
                        return Err(1);
                    } else {
                        self.left = Some(Rc::new(RefCell::new(Self {
                            c: Some(c),
                            value: 0,
                            left: None,
                            right: None,
                        })))
                    };
                    Ok(())
                }
                '1' => {
                    if self.right.is_some() {
                        return Err(1);
                    } else {
                        self.right = Some(Rc::new(RefCell::new(Self {
                            c: Some(c),
                            value: 0,
                            left: None,
                            right: None,
                        })))
                    }
                    Ok(())
                }
                _ => {
                    panic!()
                }
            }
        } else {
            match code[0] {
                '0' => {
                    if self.left.is_none() {
                        self.left = Some(Rc::new(RefCell::new(Self {
                            c: None,
                            value: 0,
                            left: None,
                            right: None,
                        })))
                    }
                    Rc::get_mut(self.left.as_mut().unwrap())
                        .unwrap()
                        .get_mut()
                        .new_from_code(c, code.drain(1..).collect())
                }
                '1' => {
                    if self.right.is_none() {
                        self.right = Some(Rc::new(RefCell::new(Self {
                            c: None,
                            value: 0,
                            left: None,
                            right: None,
                        })))
                    }
                    Rc::get_mut(self.right.as_mut().unwrap())
                        .unwrap()
                        .get_mut()
                        .new_from_code(c, code.drain(1..).collect())
                }
                _ => {
                    panic!()
                }
            }
        }
    }
}

impl PartialEq for BTree {
    fn eq(&self, other: &Self) -> bool {
        //self.value == other.value
        //&&
        self.c == other.c
            && ((self.left.as_ref() == other.left.as_ref()
                && self.right.as_ref() == other.right.as_ref())
                || (self.left.as_ref() == other.right.as_ref()
                    && self.right.as_ref() == other.left.as_ref()))
    }
}

impl PartialOrd for BTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

type PBTree = Rc<RefCell<BTree>>;

fn new_huffman_tree(mut ll: Vec<PBTree>) -> Vec<Vec<PBTree>> {
    if ll.len() == 1 {
        return vec![ll];
    }

    ll.sort_by(|a, b| a.partial_cmp(b).unwrap());

    pick_smallest(ll)
        .into_iter()
        .map(|(a, tail)| make_buffman_tree(a, tail))
        .flatten()
        .map(|v| new_huffman_tree(v))
        .flatten()
        .collect() //::<Vec<Vec<PBTree>>>()
}

fn make_buffman_tree(a: PBTree, tail: Vec<PBTree>) -> Vec<Vec<PBTree>> {
    if tail.len() == 0 {
        return vec![vec![a]];
    }

    let ll = pick_smallest(tail);

    ll.into_iter()
        .map(|(new_smallest, mut new_tail)| {
            let new = BTree::new_from_two_self_rc(a.clone(), new_smallest);
            new_tail.push(Rc::new(RefCell::new(new)));
            new_tail.sort_by(|a, b| a.partial_cmp(b).unwrap());
            new_tail
        })
        .collect()
}

fn pick_smallest(ll: Vec<PBTree>) -> Vec<(PBTree, Vec<PBTree>)> {
    if ll.len() == 0 {
        return vec![];
    }

    let key = ll[0].as_ref().borrow().value;
    let mut head: VecDeque<&Rc<RefCell<BTree>>> = VecDeque::new();
    let mut tail: Vec<&Rc<RefCell<BTree>>> = vec![];
    ll.iter().for_each(|v| {
        if v.as_ref().borrow().value == key {
            head.push_back(v)
        } else {
            tail.push(v)
        }
    });

    //dbg!(&head);
    //dbg!(&tail);

    let mut result = vec![];
    for _ in 0..head.len() {
        let this = head.drain(..1).collect::<VecDeque<_>>();
        let mut new_tail = head.iter().map(|v| (*v).clone()).collect::<Vec<_>>();
        new_tail.append(&mut tail.iter().map(|v| (*v).clone()).collect::<Vec<_>>());
        result.push((this[0].clone(), new_tail));

        head.push_back(this[0]);
    }

    result
}

fn main() {
    let f = Rc::new(RefCell::new(BTree::new('f', 2)));
    let o = Rc::new(RefCell::new(BTree::new('o', 3)));
    let r = Rc::new(RefCell::new(BTree::new('r', 4)));
    let g = Rc::new(RefCell::new(BTree::new('g', 4)));
    let e = Rc::new(RefCell::new(BTree::new('e', 5)));
    let t = Rc::new(RefCell::new(BTree::new('t', 7)));

    let aaa = new_huffman_tree(vec![f, o, r, g, e, t]);

    // aaa.iter().for_each(move |a| {
    //     dbg!(a);
    // });

    let mut a = BTree {
        c: None,
        value: 0,
        left: None,
        right: None,
    };
    a.new_from_code('f', vec!['0', '0', '0']).unwrap();
    a.new_from_code('o', vec!['0', '0', '1']).unwrap();
    a.new_from_code('r', vec!['1', '0', '0']).unwrap();
    a.new_from_code('g', vec!['1', '0', '1']).unwrap();
    a.new_from_code('e', vec!['0', '1']).unwrap();
    a.new_from_code('t', vec!['1', '1']).unwrap();

    //dbg!(a);

    dbg!(aaa
        .iter()
        .map(|v| v[0].as_ref().take() == a)
        .collect::<Vec<bool>>());

    ////////////////
    ///////////////
    //////////////

    let a = Rc::new(RefCell::new(BTree::new('a', 1)));
    let b = Rc::new(RefCell::new(BTree::new('b', 1)));
    let c = Rc::new(RefCell::new(BTree::new('c', 1)));
    let d = Rc::new(RefCell::new(BTree::new('d', 3)));
    let e = Rc::new(RefCell::new(BTree::new('e', 3)));
    let f = Rc::new(RefCell::new(BTree::new('f', 6)));
    let g = Rc::new(RefCell::new(BTree::new('g', 6)));

    let tree0s = new_huffman_tree(vec![a, b, c, d, e, f, g]);

    let mut tree1 = BTree {
        c: None,
        value: 0,
        left: None,
        right: None,
    };

    tree1
        .new_from_code('a', vec!['0', '0', '0', '0', '0'])
        .unwrap();
    tree1
        .new_from_code('b', vec!['0', '0', '0', '0', '1'])
        .unwrap();
    tree1.new_from_code('c', vec!['0', '0', '0', '1']).unwrap();
    tree1.new_from_code('d', vec!['0', '0', '1']).unwrap();
    tree1.new_from_code('e', vec!['0', '1']).unwrap();
    tree1.new_from_code('f', vec!['1', '0']).unwrap();
    tree1.new_from_code('g', vec!['1', '1']).unwrap();

    // dbg!(tree0s
    //     .iter()
    //     .map(|v| v[0].as_ref().take() == tree1)
    //     .collect::<Vec<bool>>());

    //////////////////////////////////////////////////////

    let mut tree2 = BTree {
        c: None,
        value: 0,
        left: None,
        right: None,
    };

    tree2
        .new_from_code('a', vec!['0', '1', '0', '1', '0'])
        .unwrap();
    tree2
        .new_from_code('b', vec!['0', '1', '0', '1', '1'])
        .unwrap();
    tree2.new_from_code('c', vec!['0', '1', '0', '0']).unwrap();
    tree2.new_from_code('d', vec!['0', '1', '1']).unwrap();
    tree2.new_from_code('e', vec!['1', '0']).unwrap();
    tree2.new_from_code('f', vec!['1', '1']).unwrap();
    tree2.new_from_code('g', vec!['0', '0']).unwrap();

    dbg!(tree0s
        .iter()
        .map(|v| v[0].as_ref().take() == tree2)
        .collect::<Vec<bool>>());

    //////////////////////////////////////////////////////
    let mut tree3 = BTree {
        c: None,
        value: 0,
        left: None,
        right: None,
    };

    tree3.new_from_code('a', vec!['0', '0', '0']).unwrap();
    tree3.new_from_code('b', vec!['0', '0', '1']).unwrap();
    tree3.new_from_code('c', vec!['0', '1', '0']).unwrap();
    tree3.new_from_code('d', vec!['0', '1', '1']).unwrap();
    tree3.new_from_code('e', vec!['1', '0', '0']).unwrap();
    tree3.new_from_code('f', vec!['1', '0', '1']).unwrap();
    tree3.new_from_code('g', vec!['1', '1', '0']).unwrap();

    dbg!(tree0s
        .iter()
        .map(|v| v[0].as_ref().take() == tree3)
        .collect::<Vec<bool>>());
}
