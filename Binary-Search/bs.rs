fn cut_vec(k: [i32; 8]) -> &[i32] {
    let (a, _) = k.split_at(2);
    a
}

fn main() {
    let v = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("{}", cut_vec(v));
    println!("{}", v);
}
