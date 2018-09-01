#![feature(macro_rules)]
use std::any::{Any, TypeId};
use std::ops::{Add, Sub};

struct DistanceMatrix<T> {
    length: usize,
    inner_max: Vec<Vec<Option<T>>>,
}

type Matrix<T> = Vec<Vec<Option<T>>>;

struct nil;

impl<T> DistanceMatrix<T>
where
    T: Add + Sub,
{
    fn new(input: Matrix<T>) -> Self {
        DistanceMatrix {
            length: input.len(),
            inner_max: input,
        }
    }
}

fn is_nil<T: ?Sized + Any>(o: &T) -> bool {
    TypeId::of::<i32>() == TypeId::of::<T>()
}

// new_distance_matrix!(3,[[1,2,nil],[2,3,nil],[4,nil,6]])
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
    let test0 = new_distance_matrix![[&1, &2, &3], [&4, &5, &6], [&7, &8, &_]];
    println!("{:?}", test0);
    //println!("{:?}", new_distance_matrix![[1, 2, 3], [4, 5, 6], [7, 8]]);
    println!("{:?}", is_nil(&nil {}));
}
