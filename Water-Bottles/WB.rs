pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    inner(num_bottles, 0, 0, num_exchange)
}

fn inner(water_bottles: i32, empty_bottles: i32, sum: i32, exc: i32) -> i32 {
    //println!("{},{},{},{}", water_bottles, empty_bottles, sum, exc);

    let nw = (water_bottles + empty_bottles) / exc;
    let ne = (empty_bottles + water_bottles) % exc;
    if ne < exc && nw == 0 {
        return sum + water_bottles;
    }
    inner(nw, ne, sum + water_bottles, exc)
}

fn main() {
    assert_eq!(num_water_bottles(15, 4), 19);
    assert_eq!(num_water_bottles(9, 3), 13);
    assert_eq!(num_water_bottles(5, 5), 6);
    assert_eq!(num_water_bottles(2, 3), 2);
}
