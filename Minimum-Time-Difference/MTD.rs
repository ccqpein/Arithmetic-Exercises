fn find_min_difference(times: Vec<&str>) -> i32 {
    let time_pair = times
        .iter()
        .map(|s| {
            s.split(":")
                //.map(|x| if x == "00" { "24" } else { x })
                .collect()
        })
        .collect::<Vec<Vec<&str>>>();

    let mut pure_time = time_pair
        .iter()
        .map(|x| x[0].parse::<i32>().unwrap() * 60 + x[1].parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    pure_time.sort();
    let mut smallest = pure_time[0] + 60 * 24 - pure_time.last().unwrap();
    for ind in 0..pure_time.len() - 1 {
        if pure_time[ind + 1] - pure_time[ind] < smallest {
            smallest = pure_time[ind + 1] - pure_time[ind]
        }
    }

    smallest
}

fn main() {
    dbg!(find_min_difference(vec![
        "22:27", "18:42", "09:57", "09:24", "09:26"
    ]));
}
