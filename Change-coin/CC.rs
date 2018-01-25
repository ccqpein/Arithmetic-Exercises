use std::collections::HashMap;

fn changeCoin(value: i32, coinL: &[i32], result: &mut HashMap<i32, i32>) -> i32 {
    if coinL.contains(&value) {
        return 1;
    }
    if let Some(a) = result.get(&value) {
        return *a;
    }

    let mut tempresult: Vec<i32> = Vec::new();
    for c in &coinL {
        if *c <= value {
            tempResult.push(1 + changeCoin(value - 1, coinL, result));
        }
    }

    let mut tempmin = tempresult[0];
    for tt in &tempresult {
        if *tt <= tempmin {
            tempmin = *tt;
        }
    }

    result.insert(value, tempmin);
    return tempmin;
}

fn main() {
    let coinList = [1, 5, 10, 25];
    let mut resultM: HashMap<i32, i32>;

    println!("{}", changeCoin(16));
}
