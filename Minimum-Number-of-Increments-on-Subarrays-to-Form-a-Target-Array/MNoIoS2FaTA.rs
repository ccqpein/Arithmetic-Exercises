pub fn min_number_operations(mut target: Vec<i32>) -> i32 {
    //operation_helper(&target, 0)

    /// below is faster version
    target.insert(0, 1);
    target.push(1);
    //println!("{:?}", target);
    operation_helper2(&target)
}

/// this is easier version
fn operation_helper(arr: &[i32], base: i32) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    if arr.len() == 1 {
        return arr[0] - base;
    }

    let min = arr.iter().min().unwrap();

    min - base
        + arr
            .split(|x| *x == base)
            .map(|x| operation_helper(x, *min))
            .sum::<i32>()
}

/// this is faster version
fn operation_helper2(arr: &[i32]) -> i32 {
    let mut indx = [0, 1, 2];
    let len = arr.len();

    let mut result = 0;
    let mut level = 0;
    // after find new data can added to result, next round should frozen
    let mut frozen = false;

    loop {
        //println!("{:?}: level is {}; result is {}", indx, level, result);
        let min = *[
            arr[indx[0]] - level,
            arr[indx[1]] - level,
            arr[indx[2]] - level,
        ]
        .iter()
        .min()
        .unwrap();

        // if middle one is bigest one
        if arr[indx[1]] >= arr[indx[0]] && arr[indx[1]] >= arr[indx[2]] && !frozen {
            result += arr[indx[1]] - level;
            frozen = true
        } else {
            // if min < 0, means there is a break
            if min > 0 && !frozen {
                result += min
            }
            frozen = false
        }

        level += min;

        indx[0] += 1;
        indx[1] += 1;
        indx[2] += 1;

        if indx[2] >= len {
            break;
        }
    }

    result
}

fn main() {
    assert_eq!(3, min_number_operations(vec![1, 2, 3, 2, 1]));
    assert_eq!(4, min_number_operations(vec![3, 1, 1, 2]));
    assert_eq!(7, min_number_operations(vec![3, 1, 5, 4, 2]));
    assert_eq!(1, min_number_operations(vec![1, 1, 1, 1]));
    assert_eq!(8, min_number_operations(vec![3, 4, 2, 5, 6]));
    assert_eq!(5, min_number_operations(vec![5, 5, 3]));

    //dbg!(min_number_operations(temp::big_vec_input().unwrap()));
}
