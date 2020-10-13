use std::io::{Error, ErrorKind, Result};
use std::ops::{Add, AddAssign};

#[derive(Debug)]
struct BIT<T> {
    inner: Vec<T>,
}

/// https://zh.wikipedia.org/wiki/树状数组
impl<T> BIT<T>
where
    T: Add + Default + Clone + Copy + AddAssign,
{
    fn new(a: &Vec<T>) -> Self {
        let mut inner = vec![Default::default(); a.len() + 1];

        for i in 1..=a.len() {
            inner[i] = a[i - 1];
            let mut j = i as i32 - 2;
            while j >= i as i32 - ((i as i32) & (-(i as i32))) {
                inner[i] += a[j as usize];
                j -= 1;
            }
        }

        Self { inner }
    }

    fn update(&mut self, mut ind: usize, delta: T) -> Result<()> {
        let len = self.inner.len();
        if ind >= len {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "ind beyond inner vec length",
            ));
        }

        while ind < len {
            self.inner[ind] += delta;
            ind += ((ind as i32) & (-(ind as i32))) as usize
        }

        Ok(())
    }

    fn sum(&self, ind: usize) -> Result<T> {
        let len = self.inner.len();
        if ind >= len {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "ind beyond inner vec length",
            ));
        }

        let mut result = Default::default();
        let mut ind = ind as i32;
        while ind > 0 {
            result += self.inner[ind as usize];
            ind -= (ind as i32) & (-(ind as i32))
        }

        Ok(result)
    }
}

fn main() {
    let mut a = BIT::new(&vec![3, 2, -4, 5, 7, 3, -2, 4, 7, 1, 3, 5, 1, 6, -2]);
    dbg!(&a);
    println!("{:?}", a.sum(14));

    a.update(13, 2).unwrap();
    dbg!(&a);
}
