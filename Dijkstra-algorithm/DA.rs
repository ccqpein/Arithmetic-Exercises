//#![feature(macro_rules)]
//use std::any::{Any, TypeId};
use std::cmp::{Ordering, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Deref, Sub};

/*
{start: {to: {distance, path}}}
 */
type Detail<Value> = (Value, Vec<&'static str>);
type Matrix<Value> = HashMap<&'static str, HashMap<&'static str, Detail<Value>>>;

/*
impl<T> Deref for Detail<T> {
    type Target = (T, Vec<&'static str>);

    fn deref(&self) -> &Self {
        &(*self.0, *self.1)
    }
}*/

struct DistanceMatrix<T> {
    length: usize,
    inner_max: Matrix<T>,
}

impl<T> DistanceMatrix<T>
where
    T: Add<Output = T> + Sub + Ord + Copy,
{
    fn new(input: Matrix<T>) -> Self {
        DistanceMatrix {
            length: input.len(),
            inner_max: input,
        }
    }

    fn path(&mut self, start: &'static str, to: &'static str) -> Option<Vec<&'static str>> {
        match self.inner_max.get(start) {
            Some(data) => match data.get(to) {
                Some((_, p)) => return Some(p.clone()),
                None => return None,
            },
            None => return None,
        }
    }

    fn val_between(&mut self, start: &'static str, to: &'static str) -> Option<T> {
        match self.inner_max.get(start) {
            Some(data) => match data.get(to) {
                Some((v, _)) => return Some(*v),
                None => return None,
            },
            None => return None,
        }
    }

    fn new_dijkstra_on(&mut self, start: &'static str) {
        let mut all_notes = self
            .inner_max
            .keys()
            .map(|x| *x)
            .collect::<HashSet<&'static str>>();
        all_notes.remove(start);

        let mut smallest_nodes_connected: (&'static str, Detail<T>);
        {
            let this_nodes_data = match self.inner_max.get(start) {
                Some(data) => data.clone(),
                None => {
                    println!("{}", "do not have this node");
                    return;
                }
            };

            // find shortest node of start
            let (a, b) = this_nodes_data
                .iter()
                .map(|x| x)
                .min_by_key(|(_, v)| v.0)
                .unwrap();
            smallest_nodes_connected = (*a, b.clone());
        }

        all_notes.remove(smallest_nodes_connected.0);

        let mut distance_to_now: T;
        let mut path_to_now: Vec<&'static str>;

        println!("{:?}", smallest_nodes_connected.0);
        while (!all_notes.is_empty()) {
            let distance_to_now: T;
            let path_to_now: Vec<&'static str>;

            {
                distance_to_now = self.val_between(start, smallest_nodes_connected.0).unwrap();
                path_to_now = self.path(start, smallest_nodes_connected.0).unwrap();
            }

            let this_nodes_data = match { self.inner_max.get_mut(smallest_nodes_connected.0) } {
                Some(data) => data.clone(),
                None => {
                    println!("{}", "do not have this node");
                    return;
                }
            };

            //update start node
            for (k, v) in this_nodes_data.iter() {
                if let Some(v_old) = self.val_between(start, k) {
                    if (distance_to_now + v.0) < { v_old } {
                        let mut new_path = path_to_now.clone();
                        new_path.push(k);
                        self.inner_max
                            .get_mut(start)
                            .unwrap()
                            .insert(k, (distance_to_now + v.0, new_path));
                    }
                } else {
                    let mut new_path = path_to_now.clone();
                    new_path.push(k);
                    self.inner_max
                        .get_mut(start)
                        .unwrap()
                        .insert(k, (distance_to_now + v.0, new_path));
                }
            }

            all_notes.remove(smallest_nodes_connected.0);
        }
    }
}

//struct nil;
// type check function template
/* 
fn is_nil<T: ?Sized + Any>(o: &T) -> bool {
    TypeId::of::<i32>() == TypeId::of::<T>()
}*/

// new_distance_matrix!()
// cannot check type in macro body. so do not need this macro
/*
macro_rules! new_distance_matrix {
    ($([$($x:expr),*]),*) => {{
        vec![$(vec![$(
            if is_nil($x) {
                None
            }else{
                Some($x)
            }
        ),*]),*]
    }};
    ($($x:expr),*) => {{

    }};
}*/

fn main() {
    //let test0: Matrix<u32>;
    //println!("{:?}", test0);
    //println!("{:?}", new_distance_matrix![[1, 2, 3], [4, 5, 6], [7, 8]]);

    let mut testcaseM1: Matrix<u32> = HashMap::new();
    testcaseM1.insert(
        "a",
        [("b", (6 as u32, vec![])), ("c", (3 as u32, vec![]))]
            .iter()
            .cloned()
            .collect(),
    );
    testcaseM1.insert(
        "b",
        [
            ("a", (6 as u32, vec![])),
            ("c", (2 as u32, vec![])),
            ("d", (5 as u32, vec![])),
        ]
            .iter()
            .cloned()
            .collect(),
    );
    println!("{:?}", testcaseM1);

    let mut testcase1: DistanceMatrix<u32>;
    testcase1 = DistanceMatrix {
        length: 4,
        inner_max: testcaseM1,
    };

    //Option<(&&str, &(u32, std::vec::Vec<u32>))>
    println!("{:?}", testcase1.new_dijkstra_on("a"));
}
