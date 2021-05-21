pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
    let mut cache = base_costs
        .iter()
        .map(|base| cost_cal(*base, &topping_costs, target))
        .fold(vec![], |mut acc, mut x| {
            acc.append(&mut x);
            acc
        });
    cache.sort();
    //dbg!(&cache);
    last_step(&cache, target)
}

fn cost_cal(base: i32, toppings: &[i32], target: i32) -> Vec<i32> {
    if base >= target {
        return vec![base];
    }

    if toppings.len() == 0 {
        return vec![base];
    }

    let mut result = vec![];
    result.append(&mut cost_cal(base, &toppings[1..], target));
    result.append(&mut cost_cal(base + toppings[0], &toppings[1..], target));
    result.append(&mut cost_cal(
        base + toppings[0] * 2,
        &toppings[1..],
        target,
    ));

    //result.sort();
    result
}

fn last_step(result: &[i32], target: i32) -> i32 {
    let mut diff = i32::max_value();
    let mut v = 0;
    for d in result {
        if (d - target).abs() < diff || v == *d {
            diff = (d - target).abs();
            v = *d;
            continue;
        } else {
            break;
        }
    }
    v
}

fn main() {
    dbg!(closest_cost(vec![1, 7], vec![3, 4], 10)); // 10
    dbg!(closest_cost(vec![2, 3], vec![4, 5, 100], 18)); // 17
    dbg!(closest_cost(vec![3, 10], vec![2, 5], 9)); // 8
    dbg!(closest_cost(vec![10], vec![1], 1)); // 10
}
