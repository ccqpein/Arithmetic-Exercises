use std::collections::VecDeque;

struct ProductOfNumbers {
    inner: VecDeque<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            inner: VecDeque::with_capacity(40000),
        }
    }

    fn add(&mut self, num: i32) {
        self.inner.push_front(num);
    }

    fn get_product(&self, k: i32) -> i32 {
        //self.inner.range(..k as usize).product()
        let mut result = 1;
        for ind in 0..k as usize {
            result *= self.inner[ind];
        }
        result
    }
}

fn main() {}
