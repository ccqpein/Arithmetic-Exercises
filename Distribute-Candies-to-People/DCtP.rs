pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    let mut result = vec![0; num_people as usize];
    let mut count = 1;
    let mut candies = candies;
    for ind in (0..num_people as usize).cycle() {
        //println!("candies left: {}, count: {}, ind: {}", candies, count, ind);
        if count <= candies {
            result[ind] += count
        } else {
            result[ind] += candies;
            break;
        }
        candies -= count;
        count += 1;
    }
    result
}

fn main() {
    dbg!(distribute_candies(7, 4));
    dbg!(distribute_candies(10, 3));
}
