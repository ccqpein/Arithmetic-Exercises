pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
    // let a = vec![vec![0, 0]];
    // a.append(&mut nums);
    nums.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

    let mut nums = nums.into_iter();

    let [x, mut y] = nums.next().unwrap()[..] else {
        unreachable!()
    };
    let mut result = 1 + y - x;

    for a in nums {
        //dbg!(&result);
        //dbg!(&a);
        if a[0] <= y {
            if a[1] >= y {
                result += a[1] - y;
                y = a[1];
            }
        } else {
            result += 1 + a[1] - a[0];
            //x = a[0];
            y = a[1];
        }
    }

    result
}

fn main() {
    assert_eq!(
        7,
        number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]])
    );

    assert_eq!(7, number_of_points(vec![vec![1, 3], vec![5, 8]]));

    assert_eq!(
        9,
        number_of_points(vec![
            vec![2, 3],
            vec![3, 9],
            vec![5, 7],
            vec![4, 10],
            vec![9, 10]
        ])
    );
}
