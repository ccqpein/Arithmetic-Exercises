pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    use std::collections::HashMap;
    let ss = s
        .into_bytes()
        .into_iter()
        .map(|x| x as i32)
        .collect::<Vec<_>>();

    let mut table = HashMap::new();
    let mut bucket = vec![0; ss.len()];
    shifts.into_iter().for_each(|x| {
        *table.entry((x[0], x[1])).or_insert(0) += if x[2] == 0 { -1 } else { 1 };
        // for ii in x[0]..=x[1] {
        //     bucket[ii as usize] += if x[2] == 0 { -1 } else { 1 };
        // }
    });

    table.into_iter().for_each(|(k, v)| {
        for ii in k.0..=k.1 {
            bucket[ii as usize] += v;
        }
    });
    //dbg!(&bucket);
    //dbg!(&ss);
    let xx = ss
        .into_iter()
        .zip(bucket)
        .map(|(mut b, mut offset)| {
            offset = offset % 26;
            offset = ((b - 97 + offset) + 26) % 26;
            //dbg!(offset);
            b = offset + 97;
            b as u8
        })
        .collect();

    String::from_utf8(xx).unwrap()
}

fn main() {
    // assert_eq!(
    //     shifting_letters(
    //         "abc".to_string(),
    //         vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
    //     ),
    //     "ace".to_string()
    // );

    dbg!(shifting_letters(
        "abc".to_string(),
        vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]]
    ));

    // assert_eq!(
    //     shifting_letters("dztz".to_string(), vec![vec![0, 0, 0], vec![1, 1, 1]]),
    //     "catz".to_string()
    // );

    dbg!(shifting_letters(
        "dztz".to_string(),
        vec![vec![0, 0, 0], vec![1, 1, 1]]
    ));
}
