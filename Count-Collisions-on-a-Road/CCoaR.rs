pub fn count_collisions(directions: String) -> i32 {
    let directions = directions.chars().collect::<Vec<char>>();
    let (mut left, mut right) = (0, directions.len() - 1);

    if right == 0 {
        return 0;
    }

    while left < directions.len() && directions[left] == 'L' {
        left += 1;
    }

    while right > 0 && directions[right] == 'R' {
        right -= 1;
    }

    dbg!(left);
    dbg!(right);

    directions[left..=right]
        .iter()
        .filter(|c| **c != 'S')
        .count() as i32
}

fn main() {
    dbg!(count_collisions(String::from("RLRSLL")));
    dbg!(count_collisions(String::from("LLRR")));
    dbg!(count_collisions(String::from("R")));
}
