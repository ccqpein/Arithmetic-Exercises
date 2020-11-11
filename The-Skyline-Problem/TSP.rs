pub fn get_skyline_old(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // PROBLEM here, too big slice
    let mut bucket: Vec<(i32, i32)> = vec![(0, 0); 2147483648]; // [(start(0)|end(1), height), length]
    for building in buildings {
        bucket[building[0] as usize] = (0, building[2]);
        bucket[building[1] as usize] = (1, building[2]);
    }

    let mut result: Vec<Vec<i32>> = vec![];
    let mut last: (i32, i32) = (1, 0);
    for (i, d) in bucket.iter().enumerate() {
        if *d == (0, 0) {
            continue;
        }

        match (d.0, last.0) {
            // this is start, last is end
            (0, 1) => result.push(vec![i as i32, d.1]),

            // this is end, last is start
            (1, 0) => {
                if d.1 > last.1 {
                    result.push(vec![i as i32, last.1]);
                } else if d.1 == last.1 {
                    result.push(vec![i as i32, 0])
                }
            }

            // this is start, last is start
            (0, 0) => {
                if d.1 > last.1 {
                    result.push(vec![i as i32, d.1]);
                }
            }

            // this is end, last is end
            (1, 1) => {
                if let Some(e) = result.last() {
                    if e[1] == 0 {
                        result.pop();
                    }
                }
                result.push(vec![i as i32, 0])
            }

            _ => (),
        }
        last = d.clone();
    }

    result
}

use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
struct Heap<'a, T> {
    inner_vec: Vec<T>,
    cap: usize,
    insert_p: usize, // next position of insert element

    /// cmp func used in shift_up of insert
    /// and shift_down of delete
    //cmp_func: Box<dyn Fn(&T, &T) -> bool>,
    cmp_func: &'a dyn Fn(&T, &T) -> bool,
}

impl<'a, T> Debug for Heap<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Heap")
            .field("inner_vec", &self.inner_vec)
            .field("cap", &self.cap)
            .field("insert_p", &self.insert_p)
            .finish()
    }
}

impl<'a, T> Heap<'a, T>
where
    T: Clone + Default + Debug,
{
    // fn new(f: Box<dyn Fn(&T, &T) -> bool>) -> Self {
    //     Self {
    //         inner_vec: vec![Default::default(); 2], // first one take index 0
    //         cap: 2,
    //         insert_p: 1, // next insert index
    //         cmp_func: f,
    //     }
    // }

    fn new(f: &'a dyn Fn(&T, &T) -> bool) -> Self {
        Self {
            inner_vec: vec![Default::default(); 2], // first one take index 0
            cap: 2,
            insert_p: 1, // next insert index
            cmp_func: f,
        }
    }

    // insert new element
    fn insert(&mut self, ele: &T) {
        // if inner_vec is full, double it
        if self.insert_p == self.cap {
            self.double_cap();
        }
        self.inner_vec[self.insert_p] = ele.clone();

        // adjust heap
        self.shift_up(self.insert_p);

        // give new index
        self.insert_p += 1;
    }

    // get the first element
    fn get(&self) -> Option<T> {
        if self.insert_p == 1 {
            return None;
        }

        Some(self.inner_vec[1].clone())
    }

    // delete top element of heap
    fn delete(&mut self) -> Option<T> {
        let result = if self.get().is_none() {
            return None;
        } else {
            self.get()
        };

        self.insert_p -= 1;
        // move last element to the first
        self.inner_vec[1] = self.inner_vec[self.insert_p].clone();
        self.inner_vec[self.insert_p] = Default::default();
        self.shift_down(1); // shift down from top

        result
    }

    fn len(&self) -> usize {
        self.insert_p - 1
    }

    /*
    util functions below
     */

    fn double_cap(&mut self) {
        self.inner_vec
            .append(&mut vec![Default::default(); self.cap - 1]);
        self.cap *= 2;
        self.cap -= 1;
    }

    fn shift_up(&mut self, ind: usize) {
        let root = (ind as i32 / 2) as usize;
        if root != 0 && !self.cmp_root_child(root, ind) {
            self.swop(root, ind);
            self.shift_up(root)
        }
    }

    fn shift_down(&mut self, ind: usize) {
        let (left, right) = (ind * 2, ind * 2 + 1);

        // last branch
        if right >= self.insert_p && left < self.insert_p {
            if !self.cmp_root_child(ind, left) {
                self.swop(ind, left);
            }
            return;
        }

        if left >= self.insert_p {
            return;
        }

        let next_ind = if self.cmp_root_child(left, right) {
            self.swop(ind, left);
            left
        } else {
            self.swop(ind, right);
            right
        };

        self.shift_down(next_ind);
    }

    // switch two index elements
    fn swop(&mut self, a: usize, b: usize) {
        let aa = self.inner_vec[a].clone();
        self.inner_vec[a] = self.inner_vec[b].clone();
        self.inner_vec[b] = aa
    }

    /// id1 and id2 have to exsit
    fn cmp_root_child(&self, id1: usize, id2: usize) -> bool {
        //self.cmp_func.as_ref()(&self.inner_vec[id1], &self.inner_vec[id2])
        (self.cmp_func)(&self.inner_vec[id1], &self.inner_vec[id2])
    }
}

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut heap = Heap::new(&|a: &(i32, i32, i32), b: &(i32, i32, i32)| a.0 >= b.0);
    let mut result: Vec<Vec<i32>> = vec![];
    let mut end_cache = (0, 0); // end point and height
    for b in buildings {
        // println!(
        //     "heap: {:?},\nresult: {:?}, end_cache: {:?}, building: {:?}",
        //     heap.inner_vec, result, end_cache, b
        // );
        loop {
            match heap.get() {
                Some((h, start, end)) => {
                    //println!("this heap: {:?}, end: {}", (h, end), end);
                    // delete the passed building
                    if end < b[0] {
                        heap.delete();
                        continue;
                    }

                    if h < b[2] && b[0] == start {
                        result.pop();
                        result.push(vec![b[0], b[2]])
                    } else if h < b[2] {
                        result.push(vec![b[0], b[2]])
                    } else if end_cache.0 < b[1] && end_cache.1 > b[2] {
                        result.push(vec![end_cache.0, b[2]])
                    } else if end < b[1] && h != b[2] {
                        result.push(vec![end, b[2]])
                    }

                    // add this one
                    heap.insert(&(b[2], b[0], b[1]));
                }
                None => {
                    result.push(vec![end_cache.0, 0]);
                    result.push(vec![b[0], b[2]]);
                    heap.insert(&(b[2], b[0], b[1])); // insert height, start, endpoint
                }
            }

            if b[1] > end_cache.0 {
                end_cache = (b[1], b[2]);
            }
            break;
        }
    }
    result.push(vec![end_cache.0, 0]);
    result.drain(1..).collect()
}

fn main() {
    // dbg!(get_skyline_old(vec![
    //     vec![2, 9, 10],
    //     vec![3, 7, 15],
    //     vec![5, 12, 12],
    //     vec![15, 20, 10],
    //     vec![19, 24, 8]
    // ]));

    // assert_eq!(
    //     vec![vec![0, 3], vec![1, 0]],
    //     get_skyline_old(vec![vec![0, 1, 3]])
    // );

    dbg!(get_skyline(vec![
        vec![2, 9, 10],
        vec![3, 7, 15],
        vec![5, 12, 12],
        vec![15, 20, 10],
        vec![19, 24, 8]
    ]));

    assert_eq!(
        vec![vec![0, 3], vec![1, 0]],
        get_skyline(vec![vec![0, 1, 3]])
    );

    assert_eq!(
        vec![vec![0, 3], vec![5, 0]],
        get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]])
    );

    assert_eq!(
        vec![vec![1, 3], vec![2, 0]],
        get_skyline(vec![vec![1, 2, 1], vec![1, 2, 2], vec![1, 2, 3]])
    );

    assert_eq!(
        vec![
            vec![3, 8],
            vec![7, 7],
            vec![8, 6],
            vec![9, 5],
            vec![10, 4],
            vec![11, 3],
            vec![12, 2],
            vec![13, 1],
            vec![14, 0]
        ],
        get_skyline(vec![
            vec![3, 7, 8],
            vec![3, 8, 7],
            vec![3, 9, 6],
            vec![3, 10, 5],
            vec![3, 11, 4],
            vec![3, 12, 3],
            vec![3, 13, 2],
            vec![3, 14, 1]
        ])
    );
}
