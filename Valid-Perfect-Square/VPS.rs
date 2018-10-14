fn isPerfectSquare(num: &i32) -> bool {
    let d = 2;
    let mut init = 1;
    let mut total = 0;

    while (total < *num) {
        total += init;
        init += d;
    }

    if total == *num {
        return true;
    }

    return false;
}

fn main() {
    println!("{:?}", isPerfectSquare(&14));
    println!("{:?}", isPerfectSquare(&16))
}
