pub fn num_teams(rating: Vec<i32>) -> i32 {
    let length = rating.len();
    let cache = count_larger_smaller_pair(&rating);
    //dbg!(&cache);
    let mut result = 0;
    for i in 0..length - 2 {
        for j in i + 1..length - 1 {
            if rating[j] > rating[i] {
                result += cache[j].0
            } else {
                result += cache[j].1
            }
        }
    }

    result as i32
}

fn count_larger_smaller_pair(rating: &Vec<i32>) -> Vec<(usize, usize)> {
    let length = rating.len();
    let mut cache = vec![(0, 0); rating.len()];
    for i in 0..length - 1 {
        cache[i].0 = rating[i..length].iter().filter(|&x| *x > rating[i]).count();
        cache[i].1 = length - 1 - i - cache[i].0
    }

    cache
}

fn main() {
    assert_eq!(num_teams(vec![2, 5, 3, 4, 1]), 3);
    assert_eq!(num_teams(vec![2, 1, 3]), 0);
    assert_eq!(num_teams(vec![1, 2, 3, 4]), 4);
}
