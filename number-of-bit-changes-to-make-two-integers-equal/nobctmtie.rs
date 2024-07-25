pub fn min_changes(n: i32, k: i32) -> i32 {
    let nn = format!("{:032b}", n).chars().collect::<Vec<_>>();
    let kk = format!("{:032b}", k).chars().collect::<Vec<_>>();
    //dbg!(&nn);
    //dbg!(&kk);
    if nn.len() != kk.len() {
        return -1;
    }
    let mut result = 0;
    for i in (0..nn.len()).rev() {
        if nn[i] == '1' && kk[i] == '0' {
            result += 1;
            continue;
        }

        if nn[i] == '0' && kk[i] == '1' {
            return -1;
        }
    }
    result
}

fn main() {
    dbg!(min_changes(13, 4));
    dbg!(min_changes(13, 14));
    dbg!(min_changes(21, 21));
}
