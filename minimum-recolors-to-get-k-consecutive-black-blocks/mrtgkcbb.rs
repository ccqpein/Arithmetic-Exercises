pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    if (blocks.len() as i32) < k {
        return 0;
    }

    let blocks = blocks.chars().collect::<Vec<_>>();
    let mut w_count = blocks
        .get(0..k as usize)
        .unwrap()
        .iter()
        .filter(|c| **c == 'W')
        .count();

    //let mut the_first = if blocks[0] == 'W' { 1 } else { 0 };
    let mut min = w_count;
    for i in 1..=blocks.len() - k as usize {
        let mut c = w_count;
        //dbg!(c);
        //dbg!(i);
        if blocks[i - 1] == 'W' {
            c -= 1
        };
        if blocks[i + k as usize - 1] == 'W' {
            c += 1
        };

        if c <= min {
            min = c
        }
        w_count = c
    }
    min as i32
}

fn main() {
    assert_eq!(minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
    assert_eq!(minimum_recolors("WBWBBBW".to_string(), 2), 0);
    assert_eq!(minimum_recolors("WWBBBWBBBBBWWBWWWB".to_string(), 16), 6);
}
