use std::collections::VecDeque;
use std::rc::Rc;
use std::{borrow::Borrow, cmp::Ordering};
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug)]
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

    fn record_in_table(&self, table: &mut HashMap<char, Vec<String>>) {
        self.search_with_prefix(String::new(), table)
    }

    fn search_with_prefix(&self, pre: String, table: &mut HashMap<char, Vec<String>>) {
        match self.c {
            Some(cc) => {
                table.entry(cc).or_insert(vec![]).push(pre);
            }
            None => {
                let ll = self.left.as_ref().unwrap().clone();
                ll.as_ref()
                    .borrow()
                    .search_with_prefix(pre.clone() + "0", table);

                ll.as_ref()
                    .borrow()
                    .search_with_prefix(pre.clone() + "1", table);

                let rr = self.right.as_ref().unwrap().clone();
                rr.as_ref()
                    .borrow()
                    .search_with_prefix(pre.clone() + "1", table);

                rr.as_ref()
                    .borrow()
                    .search_with_prefix(pre.clone() + "0", table);
            }
        }
    }
}

impl PartialEq for BTree {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
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

    //dbg!(new_huffman_tree(vec![f, o, r, g, e, t]));
    let aaa = new_huffman_tree(vec![f, o, r, g, e, t]);
    //dbg!(new_huffman_tree(aaa[0].clone()));
    //dbg!(pick_smallest(aaa[0].clone()));
    //dbg!(pick_smallest(vec![f, o, r, g, e, t]));
    //dbg!(make_buffman_tree(f, vec![o, r, g, e, t]));

    //let mut table = HashMap::new();

    //dbg!(aaa.len());
    aaa.iter().for_each(move |a| {
        dbg!(a);
        //a[0].as_ref().borrow().record_in_table(&mut table);
        //dbg!(&table);
    });

    //aaa[0][0].as_ref().borrow().record_in_table(&mut table);
    //dbg!(table);
}
