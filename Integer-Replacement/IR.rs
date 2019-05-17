fn inner_func(n: i32, count: i32) -> i32 {
    if n == 1 {
        return count;
    }

    if n % 2 == 0 {
        return inner_func(n / 2, count + 1);
    } else {
        let add = inner_func(n + 1, count + 1);
        let minus = inner_func(n - 1, count + 1);

        if add <= minus {
            return add;
        } else {
            return minus;
        }
    }
}

pub fn integer_replacement(n: i32) -> i32 {
    return inner_func(n, 0);
}

fn main() {
    println!("{:?}", integer_replacement(8)); //3
    println!("{:?}", integer_replacement(7)); //4
    println!("{:?}", integer_replacement(2147483647));
}
