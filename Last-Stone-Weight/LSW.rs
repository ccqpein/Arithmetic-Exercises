pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut inner_stones = stones.clone();
    inner_stones.sort_by(|x, y| y.cmp(x));
    println!("{:?}", stones);

    if stones.len() < 2 {
        return stones[0];
    }

    let mut tail: Vec<_> = inner_stones.drain(2..).collect();

    let mut first = vec![(inner_stones[0] - inner_stones[1]).abs()];
    first.append(&mut tail);
    last_stone_weight(first)
}

fn main() {
    println!("{:?}", last_stone_weight(vec![2, 7, 4, 1, 8, 1])); //=> 1
    println!("{:?}", last_stone_weight(vec![3, 7, 8])); //=> 2
}
