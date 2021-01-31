pub fn max_satisfied(mut customers: Vec<i32>, mut grumpy: Vec<i32>, x: i32) -> i32 {
    for ind in 0..customers.len() {
        if grumpy[ind] == 1 {
            grumpy[ind] = customers[ind];
            customers[ind] = 0;
        }
    }
    //dbg!(&grumpy);
    let re: i32 = customers.iter().sum();
    let x: i32 = (0..(customers.len() - x as usize + 1))
        .map(|ind| grumpy[ind..ind + x as usize].iter().sum())
        .max()
        .unwrap();
    //dbg!(&x);

    re + x
}

fn main() {
    assert_eq!(
        max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        ),
        16
    );
}
