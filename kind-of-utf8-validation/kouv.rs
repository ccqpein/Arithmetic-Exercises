/// some special utf8
/// 01000000
/// 11000000 (means two bytes) 10xxxxxx
/// 11100000 (means three bytes) 10xxxxxx 10xxxxxx

#[derive(PartialEq, Eq, Debug)]
enum State {
    ZeroOne,
    OneZero,
    MoreBytes(u8),
    Error,
}

fn get_state(x: &u8) -> State {
    let a = (0b10000000 & *x) != 0;
    let b = (0b01000000 & *x) != 0;

    match (a, b) {
        (true, true) => {
            let mut all = 2;
            for i in (0..6).rev() {
                if (1 << i) & *x != 0 {
                    all += 1
                } else {
                    break;
                }
            }
            State::MoreBytes(all)
        }
        (true, false) => State::OneZero,
        (false, true) => State::ZeroOne,
        (false, false) => State::Error,
    }
}

fn validation(inputs: &[u8]) -> Result<bool, String> {
    if inputs.len() == 0 {
        return Ok(true);
    }

    inputs.iter().for_each(|b| {
        println!("Byte {:08b}", b);
    });
    println!("");

    let more_bytes = match get_state(&inputs[0]) {
        State::ZeroOne => return validation(&inputs[1..]),
        State::OneZero => return Ok(false),
        State::MoreBytes(n) => n,
        State::Error => return Err("error".to_string()),
    };

    if more_bytes as usize > inputs.len() {
        return Ok(false);
    }

    if inputs
        .get(1..(more_bytes as usize))
        .unwrap()
        .iter()
        .any(|x| {
            if let State::OneZero = get_state(x) {
                false
            } else {
                true
            }
        })
    {
        return Ok(false);
    }

    validation(&inputs[(more_bytes as usize)..])
}

fn main() {
    //println!("{:?}", 0b11111111)
    assert_eq!(get_state(&0b10000000), State::OneZero);
    assert_eq!(get_state(&0b10100000), State::OneZero);
    assert_eq!(get_state(&0b10110000), State::OneZero);
    assert_eq!(get_state(&0b01110000), State::ZeroOne);
    assert_eq!(get_state(&0b01000000), State::ZeroOne);
    assert_eq!(get_state(&0b11110000), State::MoreBytes(4));

    let testcases = vec![
        (false, vec![0b01000000, 0b10000000]),
        (false, vec![0b10000000]),
        (false, vec![0b11000000]),
        (true, vec![0b11000000, 0b10101111]),
        (true, vec![0b01000000, 0b11100000, 0b10101111, 0b10101111]),
        (
            true,
            vec![0b01000000, 0b11100000, 0b10101111, 0b10101111, 0b01111111],
        ),
    ];

    for (ind, (res, testcase)) in testcases.into_iter().enumerate() {
        println!("");
        println!("test case {}", ind + 1);
        assert_eq!(validation(&testcase), Ok(res));
    }
}
