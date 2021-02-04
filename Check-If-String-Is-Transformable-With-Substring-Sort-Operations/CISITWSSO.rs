use std::collections::VecDeque;

pub fn is_transformable(s: String, t: String) -> bool {
    //let len = s.len();
    let mut cache = vec![VecDeque::new(); 10];

    for (ind, v) in s.chars().enumerate() {
        cache[v.to_string().parse::<usize>().unwrap()].push_back(ind)
    }

    for x in t.chars() {
        println!("{:?}", cache);
        let ind: usize = x.to_string().parse().unwrap();
        if cache[ind].len() == 0 {
            return false;
        }

        for y in 0..ind {
            if cache[y].len() > 0 && cache[y][0] < cache[ind][0] {
                return false;
            }
        }

        cache[ind].pop_front();
    }

    true
}

fn main() {
    assert!(is_transformable("84532".to_string(), "34852".to_string()));
    assert!(is_transformable("34521".to_string(), "23415".to_string()));
}
