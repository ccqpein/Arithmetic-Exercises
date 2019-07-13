pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut result = [0 as i32; 20000];
    for book in bookings {
        for ind in (book[0] - 1)..book[1] {
            result[ind as usize] = result[ind as usize] + book[2]
        }
    }

    { result.to_vec() }.drain(..n as usize).collect()
}

fn main() {
    let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
    println!("{:?}", corp_flight_bookings(bookings, 5)); //=> [10,55,45,25,25]
}
