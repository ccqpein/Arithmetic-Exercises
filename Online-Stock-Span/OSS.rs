struct StockSpanner {
    boundary: Vec<(usize, i32)>,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self {
            boundary: vec![],
            count: 0,
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.count += 1;
        self.boundary.push((self.count, price));
        self.boundary.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let pos = self
            .boundary
            .iter()
            .rposition(|(_, v)| *v == price)
            .unwrap();

        self.boundary = self.boundary.drain(pos..).collect();
        println!("{:?}", self.boundary);
        if let Some((ind, _)) = self.boundary.iter().filter(|(_, v)| *v > price).next() {
            (self.count - ind) as i32
        } else {
            self.count as i32
        }
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */

fn main() {
    let mut sp = StockSpanner::new();
    println!("{:?}", sp.next(100));
    println!("{:?}", sp.next(80));
    println!("{:?}", sp.next(60));
    println!("{:?}", sp.next(70));
    println!("{:?}", sp.next(60));
    println!("{:?}", sp.next(75));
    println!("{:?}", sp.next(85));

    // println!("{:?}", sp.next(31));
    // println!("{:?}", sp.next(41));
    // println!("{:?}", sp.next(48));
    // println!("{:?}", sp.next(59));
    // println!("{:?}", sp.next(79));
}
