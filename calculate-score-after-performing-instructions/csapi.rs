pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
    use std::collections::HashSet;
    let mut index = 0;
    let mut score = 0;
    let mut visited = HashSet::new();

    visited.insert(index);
    while index >= 0 && (index as usize) < instructions.len() {
        //dbg!(index);
        match instructions.get(index as usize).unwrap().as_str() {
            "add" => {
                score += *values.get(index as usize).unwrap() as i64;
                index += 1;
            }
            "jump" => index += *values.get(index as usize).unwrap(),
            _ => {}
        }

        if visited.contains(&index) {
            break;
        } else {
            visited.insert(index);
        }
    }

    score
}

fn main() {
    dbg!(calculate_score(
        vec!["jump", "add", "add", "jump", "add", "jump"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
        vec![2, 1, 3, 1, -2, -3],
    ));
}
