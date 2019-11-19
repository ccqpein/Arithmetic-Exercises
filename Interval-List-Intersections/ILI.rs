pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (mut a, mut b) = (a.iter(), b.iter());
    let mut result: Vec<Vec<i32>> = vec![];
    let (mut aa, mut bb) = (a.next(), b.next());

    while !aa.is_none() && !bb.is_none() {
        let this_a = aa.as_ref().unwrap();
        let this_b = bb.as_ref().unwrap();

        if this_a[0] <= this_b[0] {
            if this_a[1] < this_b[0] {
                aa = a.next();
            } else if this_a[1] >= this_b[0] {
                if this_a[1] < this_b[1] {
                    result.push(vec![this_b[0], this_a[1]]);
                    aa = a.next();
                } else {
                    result.push((*this_b).clone());
                    bb = b.next();
                }
            }
        } else {
            if this_a[0] > this_b[1] {
                bb = b.next();
            } else if this_a[1] >= this_b[1] {
                result.push(vec![this_a[0], this_b[1]]);
                bb = b.next();
            } else if this_a[1] < this_b[1] {
                result.push(vec![this_a[0], this_a[1]]);
                aa = a.next();
            }
        }
    }

    result
}

fn main() {
    dbg!(interval_intersection(
        vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
        vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
    ));
}
