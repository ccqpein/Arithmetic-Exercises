pub fn largest_integer(mut num: i32) -> i32 {
    let mut cache = vec![];
    while num != 0 {
        cache.push(num % 10);
        num /= 10
    }

    cache.reverse();
    //dbg!(&cache);
    helper(&mut cache, 0);

    let mut result = 0;
    for ind in 0..cache.len() {
        result += cache[ind] * 10_i32.pow((cache.len() - ind - 1) as u32)
    }
    result
}

fn helper(l: &mut Vec<i32>, ind: usize) {
    if ind == l.len() {
        return;
    }

    let mut max = ind;
    for d in ind..l.len() {
        if l[d] % 2 == l[ind] % 2 && l[d] > l[max] {
            max = d
        }
    }

    let a = l[max];
    l[max] = l[ind];
    l[ind] = a;
    helper(l, ind + 1);
}

fn main() {
    assert_eq!(largest_integer(1234), 3412);
    assert_eq!(largest_integer(65875), 87655);
}
