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

fn main() {}
