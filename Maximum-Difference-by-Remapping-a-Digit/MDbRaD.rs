fn number_to_digits(n: &i32) -> Vec<i32> {
    n.to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect::<Vec<_>>()
}

pub fn min_max_difference(num: i32) -> i32 {
    let a = number_to_digits(&num);
    let mut p = (0, a[0]);
    for x in &a {
        if *x != 9 {
            p.0 = *x;
            break;
        }
    }

    let x: i32 = a
        .iter()
        .map(|x| {
            if *x == p.0 {
                "9".to_string()
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    let y: i32 = a
        .iter()
        .map(|x| {
            if *x == p.1 {
                "0".to_string()
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap();

    x - y
}

fn main() {
    assert_eq!(min_max_difference(11891), 99009);
    assert_eq!(min_max_difference(90), 99);
}
