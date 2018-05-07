use std::io::{Error, ErrorKind};
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

fn main() {
    let a = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", range_vec(&a, 0, 1));
    println!("{:?}", range_vec(&a, 2, 6));
    println!("{:?}", range_vec(&a, 6, 6));
    println!("{:?}", range_vec(&a, 5, 6));
}
