use std::io::{Error, ErrorKind};
use std::ops::Add;
use std::result::Result;

fn range_vec<T>(nl: &Vec<T>, i: usize, j: usize) -> Result<Vec<T>, Error>
where
    T: Copy + Clone,
{
    if i >= nl.len() {
        return Err(Error::new(
            ErrorKind::Other,
            "first index larger than length",
        ));
    }

    let mut jj;

    if j > nl.len() {
        jj = nl.len() - 1;
    } else {
        jj = j - 1;
    }

    let mut ind = 0;
    let mut result: Vec<T> = vec![];

    for this in nl {
        if ind >= i && ind <= jj {
            result.push(*this);
        }
        ind += 1;
    }

    Result::Ok(result)
}

fn sum_vec<T>(nl: &Vec<T>) -> T
where
    T: Add<Output = T> + Copy + Clone,
{
    let mut result: T = *nl.first().unwrap();
    if let Some((_, last)) = nl.split_first() {
        for this in last {
            result = result + (*this);
        }
    }
    result
}

/*
fn sum_range<T>(nl: &Vec<T>, i: usize, j: usize) -> Result<T, Error>
where
    T: Add + Copy + Clone,
{

}*/

fn main() {
    let a = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", range_vec(&a, 0, 1));
    println!("{:?}", range_vec(&a, 2, 6));
    println!("{:?}", range_vec(&a, 6, 6));
    println!("{:?}", range_vec(&a, 5, 6));
    //main thread cannot use `?`
    //println!("{:?}", range_vec(&a, 2, 6)?);

    println!("{:?}", sum_vec(&a));
}
