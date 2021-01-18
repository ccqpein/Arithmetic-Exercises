use std::collections::VecDeque;
use std::iter::FromIterator;
use std::str::Chars;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut a = digits.chars();
    rec_lc(&mut a)
        .into_iter()
        .map(|dq| String::from_iter(dq))
        .filter(|s| *s != "".to_string())
        .collect()
}

fn rec_lc<'a>(s: &mut Chars<'a>) -> Vec<VecDeque<char>> {
    if let Some(c) = s.next() {
        let cache = rec_lc(s);
        match c {
            '2' => {
                //let mut cache = rec_lc(s);
                ['a', 'b', 'c']
                    .iter()
                    .map(|v| {
                        cache
                            .iter()
                            .cloned()
                            .map(|mut l| {
                                l.push_front(*v);
                                l
                            })
                            .collect::<Vec<VecDeque<char>>>()
                    })
                    .flatten()
                    .collect::<Vec<VecDeque<char>>>()
            }
            '3' => ['d', 'e', 'f']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            '4' => ['g', 'h', 'i']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            '5' => ['j', 'k', 'l']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            '6' => ['m', 'n', 'o']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            '7' => ['p', 'q', 'r', 's']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            '8' => ['t', 'u', 'v']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            '9' => ['w', 'x', 'y', 'z']
                .iter()
                .map(|v| {
                    cache
                        .iter()
                        .cloned()
                        .map(|mut l| {
                            l.push_front(*v);
                            l
                        })
                        .collect::<Vec<VecDeque<char>>>()
                })
                .flatten()
                .collect::<Vec<VecDeque<char>>>(),
            _ => panic!(c),
        }
    } else {
        vec![VecDeque::new()]
    }
}

fn main() {
    dbg!(letter_combinations("23".to_string()));
    dbg!(letter_combinations("".to_string()));
    dbg!(letter_combinations("2".to_string()));
}
