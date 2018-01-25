use std::collections::HashMap;

fn changeCoin(value: i32, coinL: &[i32], result: &mut HashMap<i32, i32>) -> i32 {
    if coinL.contains(&value) {
        return 1;
    }
    if let Some(a) = result.get(&value) {
        return *a;
    }

    let mut tempresult: Vec<i32> = Vec::new();
    for c in coinL {
        if *c <= value {
            tempresult.push(1 + changeCoin(value - 1, coinL, result));
        }
    }

    let mut tempmin = tempresult[0];
    for tt in &tempresult {
        if *tt < tempmin {
            tempmin = *tt;
        }
    }

    result.insert(value, tempmin);
    println!("{}=>{}, {:?}", value, tempmin, result);
    return tempmin;
}

fn main() {
    let coinList = [1, 5, 12, 25];
    let mut resultM: HashMap<i32, i32> = HashMap::new();

    println!("{}", changeCoin(16, &coinList, &mut resultM));
}
