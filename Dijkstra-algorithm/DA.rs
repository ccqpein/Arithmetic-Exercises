//#![feature(macro_rules)]
//use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{Add, Sub};

/*
{start: }
*/
type Matrix<T> = HashMap<&'static str, HashMap<&'static str, Option<T>>>;

struct DistanceMatrix<T> {
    length: usize,
    inner_max: Matrix<T>,
}

impl<T> DistanceMatrix<T>
where
    T: Add + Sub + Ord,
{
    fn new(input: Matrix<T>) -> Self {
        DistanceMatrix {
            length: input.len(),
            inner_max: input,
        }
    }

    fn new_dijkstra_on(&mut self, end: &'static str) {}
}

//struct nil;
// type check function template
/* 
fn is_nil<T: ?Sized + Any>(o: &T) -> bool {
    TypeId::of::<i32>() == TypeId::of::<T>()
}*/

// new_distance_matrix!()
// cannot check type in macro body. so do not need this macro
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
}

fn main() {
    let test0: Matrix<u32> = [(
        "a",
        [("b", Some(6)), ("c", Some(3))] as HashMap<&'static str, Option<u32>>,
    )];
    println!("{:?}", test0);
    //println!("{:?}", new_distance_matrix![[1, 2, 3], [4, 5, 6], [7, 8]]);
}
