fn get_value(a: &usize, b: &usize, m: &Vec<Vec<u32>>) -> u32 {
    m[*a][*b]
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn round_matrix(m: &Vec<Vec<u32>>) -> Vec<u32> {
    let length = m.len();
    let mut direction = 0;
    let mut a: usize = 0;
    let mut b: usize = 0;

    let mut resutl = vec![get_value(&a, &b, m)];

    b += 1;

    while a != 0 || b != 0 {
        if direction == 0 {
            resutl.push(get_value(&a, &b, m));
            if b == length - 1 {
                direction = 1;
                a += 1;
            } else {
                b += 1;
            }
            continue;
        }
        if direction == 1 {
            resutl.push(get_value(&a, &b, m));
            if a == length - 1 {
                direction = 2;
                b -= 1;
            } else {
                a += 1;
            }
            continue;
        }
        if direction == 2 {
            resutl.push(get_value(&a, &b, m));
            if b == 0 {
                direction = 3;
                a -= 1
            } else {
                b -= 1;
            }
            continue;
        }

        if direction == 3 {
            resutl.push(get_value(&a, &b, m));
            if a == 0 {
                break;
            } else {
                a -= 1;
            }
        }
    }
    resutl
}

fn round_matrix2(m: &Vec<Vec<u32>>) -> Vec<u32> {}

fn main() {
    let testa = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("{:?}", round_matrix(&testa));
}
