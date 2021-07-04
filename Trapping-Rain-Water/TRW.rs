pub fn trap(height: Vec<i32>) -> i32 {
    let (mut left_max, mut right_max) = (
        Vec::with_capacity(height.len()),
        Vec::with_capacity(height.len()),
    );

    let mut start = 0;
    height.iter().for_each(|n| {
        if *n > start {
            left_max.push(*n);
            start = *n;
        } else {
            left_max.push(start)
        }
    });

    // from right
    start = 0;
    height.iter().rev().for_each(|n| {
        if *n > start {
            right_max.push(*n);
            start = *n;
        } else {
            right_max.push(start)
        }
    });

    right_max.reverse(); // reverse

    //println!("left_max: {:?}, right_max: {:?}", left_max, right_max);

    (0..height.len())
        .map(|ind| left_max[ind].min(right_max[ind]) - height[ind])
        .sum()
}

fn main() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
