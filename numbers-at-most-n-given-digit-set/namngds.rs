pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    let n = num_helper(n);
    helper(
        &digits.into_iter().map(|d| d.parse().unwrap()).collect(),
        &n,
        true,
    )
}

// wrong
// fn helper(digits: Vec<i32>, n: &[i32]) -> i32 {
//     //dbg!(&digits);
//     //dbg!(n);
//     if n.len() == 1 {
//         return digits.iter().filter(|d| **d <= n[0]).count() as i32;
//     }

//     let mut result = 0;
//     let digit_len = digits.len() as f32;

//     let xx = (1..=n.len() - 1)
//         .map(|x| digit_len.powi(x as i32) as i32)
//         .sum::<i32>();

//     for _ in 0..n[0] {
//         result += xx;
//     }

//     //let mut result = result as i32;
//     result += helper(digits, &n[1..]);

//     result
// }

fn helper(digits: &Vec<i32>, n: &[i32], start_with_zero: bool) -> i32 {
    //dbg!(&digits);
    //dbg!(n);

    if n.len() == 1 {
        return (0..=n[0]).filter(|a| digits.contains(a)).count() as i32;
    }

    let mut result = 0;
    let digit_len = digits.len();

    result += (1..n[0]).filter(|a| digits.contains(a)).count() as i32
        * digit_len.pow((n.len() - 1) as u32) as i32
        + if start_with_zero {
            helper(digits, &vec![9; n.len() - 1], true)
        } else {
            0
        };

    if digits.contains(&n[0]) {
        result += helper(digits, &n[1..], false)
    }

    result
}

fn num_helper(mut n: i32) -> Vec<i32> {
    let mm = 10;
    let mut result = vec![];
    loop {
        if n % mm == n {
            result.push(n % mm);
            break;
        }
        result.push(n % mm);
        n /= 10;
    }
    result.reverse();
    result
}

fn main() {
    //dbg!(num_helper(100));
    //dbg!(num_helper(1234));

    assert_eq!(
        at_most_n_given_digit_set(
            ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            899894860
        ),
        392738517
    );

    assert_eq!(
        at_most_n_given_digit_set(
            ["1", "3", "5", "7"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            99
        ),
        20
    );

    assert_eq!(
        at_most_n_given_digit_set(
            ["1", "3", "5", "7"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            100
        ),
        20
    );

    assert_eq!(
        at_most_n_given_digit_set(
            ["1", "5", "7", "8"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            10212
        ),
        340
    );

    assert_eq!(
        at_most_n_given_digit_set(["9"].into_iter().map(|s| s.to_string()).collect(), 100),
        2
    );

    assert_eq!(
        at_most_n_given_digit_set(["9"].into_iter().map(|s| s.to_string()).collect(), 1000),
        3
    );

    assert_eq!(
        at_most_n_given_digit_set(
            ["1", "4", "9"].into_iter().map(|s| s.to_string()).collect(),
            1000000000
        ),
        29523
    );

    assert_eq!(
        at_most_n_given_digit_set(["7"].into_iter().map(|s| s.to_string()).collect(), 8),
        1
    );

    assert_eq!(
        at_most_n_given_digit_set(["9"].into_iter().map(|s| s.to_string()).collect(), 55),
        1
    );
    assert_eq!(
        at_most_n_given_digit_set(["1"].into_iter().map(|s| s.to_string()).collect(), 834),
        3
    );
}
