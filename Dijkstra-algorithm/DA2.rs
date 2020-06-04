/// DA.rs is not good, let me rewrite it
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug)]
struct Graph<T, P>
where
    T: Eq + Hash,
    P: PartialOrd,
{
    form: HashMap<T, HashMap<T, P>>,
    path_form: HashMap<T, HashMap<T, Vec<T>>>,
}

impl<T, P> Graph<T, P>
where
    T: Eq + Hash + Clone + Copy,
    P: PartialOrd + Clone + Copy + Add<Output = P>,
{
    fn new() -> Self {
        Graph {
            form: HashMap::new(),
            path_form: HashMap::new(),
        }
    }

    // don't forget insert node itself
    fn insert(&mut self, data: &(T, T, P)) {
        self.form
            .entry(data.0)
            .or_insert(HashMap::new())
            .insert(data.1, data.2);

        self.form
            .entry(data.1)
            .or_insert(HashMap::new())
            .insert(data.0, data.2);

        let mut path = if data.0 == data.1 {
            vec![data.0]
        } else {
            vec![data.0, data.1]
        };
        self.insert_path(&(data.0, data.1, &path));
        path.reverse();
        self.insert_path(&(data.1, data.0, &path));
    }

    fn distance_between(&self, x: &T, y: &T) -> Result<P, String> {
        match self.form.get(x) {
            Some(x) => match x.get(y) {
                Some(x) => Ok(*x),
                None => Err(String::from("Not Found")),
            },
            None => Err(String::from("Not Found")),
        }
    }

    fn insert_path(&mut self, path_tuple: &(T, T, &Vec<T>)) {
        self.path_form
            .entry(path_tuple.0)
            .or_insert(HashMap::new())
            .insert(path_tuple.1, path_tuple.2.clone());
    }

    fn path_between(&mut self, x: &T, y: &T) -> Option<&Vec<T>> {
        self.path_form.entry(*x).or_insert(HashMap::new()).get(y)
    }

    // update all distance from start
    fn dijkstra(&mut self, start: &T) {
        let mut rest_set = self.form.keys().cloned().collect::<HashSet<T>>();
        rest_set.remove(start);

        let mut last = *start;
        while !rest_set.is_empty() {
            // find all nodes connected with last in rest_set
            let rest_nodes = {
                rest_set
                    .iter()
                    .map(|x| (*x, self.distance_between(&last, x)))
                    .filter(|x| x.1.is_ok())
                    .map(|x| (x.0, x.1.unwrap()))
                    .collect::<Vec<(T, P)>>()
            };

            // update start to node value
            for node in rest_nodes {
                match self.distance_between(start, &node.0) {
                    Ok(d) => {
                        if self.distance_between(start, &last).unwrap() + node.1 < d {
                            self.insert(&(
                                *start,
                                node.0,
                                self.distance_between(start, &last).unwrap() + node.1,
                            ));

                            // insert path
                            let mut new_path =
                                { self.path_between(start, &last).cloned() }.unwrap();
                            new_path.push(node.0); // put new node inside
                            self.insert_path(&(*start, node.0, &new_path));
                        }
                    }
                    Err(_) => {
                        self.insert(&(
                            *start,
                            node.0,
                            self.distance_between(start, &last).unwrap() + node.1,
                        ));

                        // insert path
                        let mut new_path = { self.path_between(start, &last).cloned() }.unwrap();
                        new_path.push(node.0); // put new node inside
                        self.insert_path(&(*start, node.0, &new_path));
                    }
                }
            }

            // search from rest_set again because value maybe changed
            let next = {
                let mut cache = rest_set
                    .iter()
                    .map(|x| (x, self.distance_between(start, x)))
                    .filter(|x| x.1.is_ok())
                    .map(|x| (x.0, x.1.unwrap()))
                    .collect::<Vec<(&T, P)>>();

                // find nearest one
                cache.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                *cache.first().unwrap().0
            };

            // next loop start from next
            rest_set.remove(&next);
            last = next;
        }
    }
}

fn main() {
    let mut testcase0: Graph<i32, i32> = Graph::new();
    let set = vec![
        (1, 1, 0),
        (2, 2, 0),
        (3, 3, 0),
        (4, 4, 0),
        (5, 5, 0),
        (6, 6, 0),
        (1, 2, 7),
        (1, 3, 9),
        (1, 6, 14),
        (2, 3, 10),
        (2, 4, 15),
        (3, 6, 2),
        (3, 4, 11),
        (4, 5, 6),
        (5, 6, 9),
    ];

    for s in set {
        testcase0.insert(&s);
    }

    testcase0.dijkstra(&1);
    println!("{:?}", testcase0.form.get(&1));
    println!("{:?}", testcase0.path_form.get(&1));

    //////
    let mut testcase1: Graph<char, i32> = Graph::new();
    let set = vec![
        ('A', 'A', 0),
        ('B', 'B', 0),
        ('C', 'C', 0),
        ('D', 'D', 0),
        ('E', 'E', 0),
        ('F', 'F', 0),
        //
        ('A', 'B', 6),
        ('A', 'C', 3),
        ('B', 'C', 2),
        ('B', 'D', 5),
        ('C', 'D', 3),
        ('C', 'E', 4),
        ('D', 'E', 2),
        ('D', 'F', 3),
        ('E', 'F', 5),
    ];

    for s in set {
        testcase1.insert(&s);
    }
    testcase1.dijkstra(&'A');
    println!("{:?}", testcase1.form.get(&'A'));
    println!("{:?}", testcase1.path_form.get(&'A'));
}
