pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut flag = bills[0] == 5 && bills[1] < 20;

    let mut a = 0;
    let mut b = 0;

    for x in bills {
        if !flag {
            return flag;
        }

        match x {
            5 => {
                a += 1;
                continue;
            }
            10 => {
                flag = a > 0;
                a -= 1;
                b += 1;
                continue;
            }
            _ => (),
        }

        if b > 0 {
            flag = a > 0;
            a -= 1;
            b -= 1;
            continue;
        }

        // rest situation
        flag = a > 2;
        a -= 3;
    }
    flag
}

fn main() {}
