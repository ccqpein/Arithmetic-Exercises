pub fn find_integers(num: i32) -> i32 {
    let (mut res, mut x, mut y, mut num) = (0, 1, 2, num + 1);
    while num != 0 {
        if (num & 1) != 0 && (num & 2) != 0 {
            res = 0;
        }
        res += x * (num & 1);
        num >>= 1;
        let cache = x + y;
        x = y;
        y = cache;
    }
    res
}
