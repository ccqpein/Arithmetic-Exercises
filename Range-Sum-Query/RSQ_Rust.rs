use std::io::{Error, ErrorKind};
use std::ops::Add;
use std::result::Result;

fn range_vec<T>(nl: &Vec<T>, i: usize, j: usize) -> Result<Vec<T>, Error>
where
    T: Copy + Clone,
{
    if i >= nl.len() {
        return Result::Err(Error::new(
            ErrorKind::Other,
            "first index larger than length",
        ));
    }

    let mut jj;

    if j > nl.len() {
        jj = nl.len();
    } else {
        jj = j;
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

fn sum_range<T>(nl: &Vec<T>, i: usize, j: usize) -> Result<T, Error>
where
    T: Add<Output = T> + Copy + Clone,
{
    match range_vec(nl, i, j) {
        Result::Ok(temp_l) => return Result::Ok(sum_vec(&temp_l)),
        Result::Err(e) => return Result::Err(e),
    }
}

fn main() {
    let a = vec![-2, 0, 3, -5, 2, -1];
    println!("{:?}", range_vec(&a, 0, 1));
    println!("{:?}", range_vec(&a, 2, 6));
    println!("{:?}", range_vec(&a, 6, 6));
    println!("{:?}", range_vec(&a, 5, 6));
    //main thread cannot use `?`
    //println!("{:?}", range_vec(&a, 2, 6)?);

    println!("{:?}", sum_vec(&a));

    println!("{:?}", sum_range(&a, 0, 2));
    println!("{:?}", sum_range(&a, 2, 5));
    println!("{:?}", sum_range(&a, 0, 5));
}
