#![feature(box_into_inner)]

#[derive(Debug)]
struct List {
    v: i32,
    next: Option<Box<Self>>,
}

impl List {
    fn new(mut input: Vec<i32>) -> Option<Self> {
        let mut this;
        let mut tail = None;
        loop {
            if let Some(i) = input.pop() {
                this = Self { v: i, next: tail };
                tail = Some(Box::new(this));
            } else {
                return Some(Box::into_inner(tail.unwrap()));
            };
        }
    }

    fn new_helper(mut input: Vec<Self>) -> Result<Self, String> {
        let mut this = match input.pop() {
            Some(tail) => tail,
            None => return Err("are you fucking kidding me?".to_string()),
        };

        loop {
            if let Some(mut tail) = input.pop() {
                tail.next = Some(Box::new(this));
                this = tail;
            } else {
                return Ok(this);
            };
        }
    }

    fn for_dear_uncle_shi(self, k: usize) -> Self {
        let mut head = self;
        let mut next = head.next.take();
        let mut cache = vec![];
        for ind in 0..k {
            cache.push(head);
            match next {
                Some(n) => {
                    head = Box::into_inner(n);
                    next = head.next.take();
                }
                None => {
                    if ind != k - 1 {
                        return Self::new_helper(cache).unwrap();
                    } else {
                        cache.reverse();
                        return Self::new_helper(cache).unwrap();
                    }
                }
            }
        }

        head.next = next;
        cache[0].next = Some(Box::new(head.for_dear_uncle_shi(k)));
        cache.reverse();
        Self::new_helper(cache).unwrap()
    }
}

fn main() {
    // dbg!(List::new(vec![1, 2, 3, 4, 5, 6, 7, 8]));
    // dbg!(List::new_helper(vec![
    //     List { v: 3, next: None },
    //     List { v: 2, next: None },
    //     List { v: 1, next: None }
    // ]));

    let a = List::new(vec![1, 2, 3, 4, 5, 6]).unwrap();
    dbg!(a.for_dear_uncle_shi(3));
    let a = List::new(vec![1, 2, 3, 4, 5, 6]).unwrap();
    dbg!(a.for_dear_uncle_shi(4));
}
