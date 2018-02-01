fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 1 {
        return 0;
    }

    //let mut max_temp = 0;
    let mut min_temp = prices[0];
    let mut result = 0;

    for i in &prices {
        if *i - min_temp < 0 {
            min_temp = *i
        } else if *i - min_temp > result {
            result = *i - min_temp;
        } else {
            continue;
        }
    }


    if result < 0 {
        0
    } else {
        result
    }
}

fn main() {
    let test0 = Vec::new();
    let test1 = vec![2, 1, 2, 1, 0, 1, 2];
    let test2 = vec![1];
    let test3 = vec![7, 1, 5, 3, 6, 4];
    let test4 = vec![2, 4, 1];

    println!("{}", max_profit(test0));
    println!("{}", max_profit(test1));
    println!("{}", max_profit(test2));
    println!("{}", max_profit(test3));
    println!("{}", max_profit(test4));
}
