fn number_to_digits(mut num: i32) -> Vec<i32> {
    let mut result = vec![];
    loop {
        if num > 0 {
            result.push(num % 10);
            num = num / 10;
        } else {
            break;
        }
    }

    result.reverse();
    result
}

fn digits_to_number(digits: Vec<i32>) -> i32 {
    let mut i = 1;
    digits.into_iter().rev().fold(0, |acc, d| {
        let v = acc + i * d;
        i *= 10;
        v
    })
}

pub fn maximum_swap(num: i32) -> i32 {
    let mut digits = number_to_digits(num);
    digits.reverse();

    let mut st: Vec<i8> = vec![-1; digits.len()];

    let mut i = 0;
    for j in 0..digits.len() {
        if digits[i] > digits[j] {
            st[j] = i as i8;
        } else if digits[j] > digits[i] {
            i = j;
        }
    }
    for j in (0..digits.len()).rev() {
        if st[j] >= 0 {
            digits.swap(j, st[j] as usize);
            break;
        }
    }

    digits.reverse();
    digits_to_number(digits)
}

fn main() {
    dbg!(number_to_digits(2736));
    dbg!(digits_to_number(vec![2, 7, 3, 6]));
    dbg!(maximum_swap(2736));
}
