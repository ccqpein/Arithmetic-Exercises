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
        // all nodes set
        let mut all_notes = self
            .inner_max
            .keys()
            .map(|x| *x)
            .collect::<HashSet<&'static str>>();
        all_notes.remove(start);

        let mut smallest_nodes_connected: &'static str;
        {
            let this_nodes_data = match self.inner_max.get(start) {
                Some(data) => data.clone(), // need clone here
                None => {
                    println!("{}", "do not have this node");
                    return;
                }
            };

            // find smallest node
            let (a, _) = this_nodes_data
                .iter()
                .map(|x| x)
                .filter(|x| all_notes.contains(x.0)) // must nodes in all nodes.
                .min_by_key(|(_, v)| v.0)
                .unwrap();
            smallest_nodes_connected = *a;
        }

        //all_notes.remove(smallest_nodes_connected);

        //println!("{:?}", smallest_nodes_connected);
        while (!all_notes.is_empty()) {
            let distance_to_now: T;
            let path_to_now: Vec<&'static str>;

            all_notes.remove(smallest_nodes_connected);
            {
                distance_to_now = self.val_between(start, smallest_nodes_connected).unwrap();
                path_to_now = self.path(start, smallest_nodes_connected).unwrap();
            }

            let this_nodes_data = match { self.inner_max.get_mut(smallest_nodes_connected) } {
                Some(data) => data.clone(),
                None => {
                    println!("{}", "do not have this node");
                    return;
                }
            };

            //update start node
            let mut this_round_result: Vec<(&'static str, T)> = vec![];
            for (k, v) in this_nodes_data.iter() {
                // cannot be start node
                if *k == start {
                    continue;
                }

                if let Some(v_old) = self.val_between(start, k) {
                    if (distance_to_now + v.0) < v_old {
                        let mut new_path = path_to_now.clone();
                        new_path.push(k);
                        self.inner_max
                            .get_mut(start)
                            .unwrap()
                            .insert(k, (distance_to_now + v.0, new_path));
                        this_round_result.push((*k, distance_to_now + v.0));
                    } else {
                        // if old value is smaller one, dont change path
                        // but push result in this_round_result
                        this_round_result.push((*k, v_old));
                    }
                } else {
                    let mut new_path = path_to_now.clone();
                    new_path.push(k);
                    self.inner_max
                        .get_mut(start)
                        .unwrap()
                        .insert(k, (distance_to_now + v.0, new_path));
                    this_round_result.push((*k, distance_to_now + v.0));
                }
            }

            // next smallest node
            smallest_nodes_connected = if let Some(a) = this_round_result
                .iter()
                .filter(|x| all_notes.contains(x.0))
                .min_by_key(|(_, v)| v)
            {
                a.0
            } else {
                ""
            };

            //println!("{:?}", smallest_nodes_connected);
        }
    }
}

//struct nil;
// type check function template
/* 
fn is_nil<T: ?Sized + Any>(o: &T) -> bool {
    TypeId::of::<i32>() == TypeId::of::<T>()
}*/

// cannot check type in macro body. so do not need this macro
/*
(
 ["a",("b",6 ["a","b"]),("c",3,["a","c"])],
 ["b",...),
)
*/

macro_rules! new_distance_matrix {
    ($([$x:expr,$(($y:expr,$z:expr,[$($p:expr),*])),*]),*) => {{
        let mut temp: Matrix<u32> = HashMap::new();
        $(
            temp.insert(
                $x,
                [$(($y,($z as u32,vec![$($p),*]))),*].iter().cloned().collect(),
            );
        )*;
        temp
    }};
}

fn main() {
    //let test0: Matrix<u32>;
    //println!("{:?}", test0);
    //println!("{:?}", new_distance_matrix![[1, 2, 3], [4, 5, 6], [7, 8]]);

    let mut test_caseM1: Matrix<u32> = HashMap::new();
    test_caseM1.insert(
        "a",
        [
            ("b", (6 as u32, vec!["a", "b"])),
            ("c", (3 as u32, vec!["a", "c"])),
        ]
            .iter()
            .cloned()
            .collect(),
    );
    test_caseM1.insert(
        "b",
        [
            ("a", (6 as u32, vec!["b", "a"])),
            ("c", (2 as u32, vec!["b", "c"])),
            ("d", (5 as u32, vec!["b", "d"])),
        ]
            .iter()
            .cloned()
            .collect(),
    );
    test_caseM1.insert(
        "c",
        [
            ("a", (3 as u32, vec!["c", "a"])),
            ("d", (3 as u32, vec!["c", "d"])),
            ("b", (2 as u32, vec!["c", "b"])),
            ("e", (4 as u32, vec!["c", "e"])),
        ]
            .iter()
            .cloned()
            .collect(),
    );
    test_caseM1.insert(
        "d",
        [
            ("b", (5 as u32, vec!["d", "b"])),
            ("e", (2 as u32, vec!["d", "e"])),
            ("c", (3 as u32, vec!["d", "c"])),
            ("f", (3 as u32, vec!["d", "f"])),
        ]
            .iter()
            .cloned()
            .collect(),
    );
    test_caseM1.insert(
        "e",
        [
            ("c", (4 as u32, vec!["e", "c"])),
            ("d", (2 as u32, vec!["e", "d"])),
            ("f", (5 as u32, vec!["e", "f"])),
        ]
            .iter()
            .cloned()
            .collect(),
    );
    test_caseM1.insert(
        "f",
        [
            ("e", (5 as u32, vec!["f", "e"])),
            ("d", (3 as u32, vec!["f", "d"])),
        ]
            .iter()
            .cloned()
            .collect(),
    );

    println!("{:?}", test_caseM1);

    let mut test_case1: DistanceMatrix<u32>;
    test_case1 = DistanceMatrix {
        length: 4,
        inner_max: test_caseM1,
    };

    //Option<(&&str, &(u32, std::vec::Vec<u32>))>
    //println!("{:?}", test_case1.new_dijkstra_on("a"));
    //println!("{:?}", test_case1.inner_max.get("a"));

    // test macro
    let macro_test_marix = new_distance_matrix!(
        ["a", ("b", 6, ["a", "b"]), ("c", 3, ["a", "c"])],
        [
            "b",
            ("a", 6, ["b", "a"]),
            ("c", 2, ["b", "c"]),
            ("d", 5, ["b", "d"])
        ],
        [
            "c",
            ("a", 3, ["c", "a"]),
            ("d", 3, ["c", "d"]),
            ("b", 2, ["c", "b"]),
            ("e", 4, ["c", "e"])
        ],
        [
            "d",
            ("b", 5, ["d", "b"]),
            ("e", 2, ["d", "e"]),
            ("c", 3, ["d", "c"]),
            ("f", 3, ["d", "f"])
        ],
        [
            "e",
            ("c", 4, ["e", "c"]),
            ("d", 2, ["e", "d"]),
            ("f", 5, ["e", "f"])
        ],
        ["f", ("e", 5, ["f", "e"]), ("d", 3, ["f", "d"])]
    );
    println!("{:?}", macro_test_marix);
}
