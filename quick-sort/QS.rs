fn quick_sort(l: Vec<u32>) -> Vec<u32> {
    if l.len() == 0 {
        return vec![];
    }

    let key = l[0];
    let mut smaller = vec![];
    let mut larger = vec![];

    for i in &l[1..] {
        if *i <= key {
            smaller.push(*i);
        } else {
            larger.push(*i);
        }
    }

    let mut result = quick_sort(smaller);

    result.push(key);
    result.append(&mut quick_sort(larger));
    result
}

fn main() {
    let result = quick_sort(vec![3, 2, 4, 1, 5, 4, 3, 5, 7, 7, 8, 0, 3, 2]);
    println!("{:?}", result);
}
