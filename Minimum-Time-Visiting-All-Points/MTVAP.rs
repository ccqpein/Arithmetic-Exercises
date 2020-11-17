pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::max;
    if points.len() == 0 {
        return 0;
    }

    let mut result = 0;
    let mut points_iter = points.iter();
    let mut last = points_iter.next().unwrap();
    loop {
        if let Some(p) = points_iter.next() {
            result += max((p[1] - last[1]).abs(), (p[0] - last[0]).abs());
            last = p;
        } else {
            break;
        }
    }
    result
}

pub fn min_time_to_visit_all_points2(points: Vec<Vec<i32>>) -> i32 {
    use std::cmp::max;
    let mut last = points[0].clone();
    points.iter().fold(0, |acc, next| {
        let a = acc + max((next[1] - last[1]).abs(), (next[0] - last[0]).abs());
        last = next.clone();
        a
    })
}

fn main() {}
