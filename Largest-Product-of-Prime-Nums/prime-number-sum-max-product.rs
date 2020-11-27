use std::collections::HashMap;

const PRIME_NUMBER: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009,
];

#[derive(Clone, Debug)]
struct Record {
    inner: (u128, Vec<u64>),
}

type Records = Vec<Record>;

impl Record {
    fn new(n: u64, v: Vec<u64>) -> Self {
        Self {
            inner: (n as u128, v),
        }
    }

    fn update_record(i: u64, other: &Self) -> Self {
        Self {
            inner: (i as u128 * other.inner.0, {
                let mut a = other.inner.1.clone();
                a.push(i);
                a
            }),
        }
    }
}

fn max_product<'a>(
    n: u64,
    table: &'a mut HashMap<u64, Records>,
    prime_n: &[u64],
) -> Option<&'a Records> {
    let empty_trick = vec![];
    match n {
        1 => None,
        _ => {
            for pn in prime_n {
                if *pn < n {
                    let records = match table.get(&(n - *pn)) {
                        Some(records) => records,
                        None => max_product(n - *pn, table, prime_n).unwrap_or(&empty_trick),
                    };

                    if let Some(record) = records.iter().find(|r| !r.inner.1.contains(pn)) {
                        let new_re = Record::update_record(*pn, record);
                        let update_re = table.entry(n).or_insert(vec![]);
                        update_re.push(new_re);
                        update_re.sort_by(|x, y| y.inner.0.partial_cmp(&x.inner.0).unwrap());
                    }
                }
            }

            table.get(&n)
        }
    }
}

fn main() {
    let mut table: HashMap<u64, Records> = HashMap::new();
    table.insert(2, vec![Record::new(2, vec![2])]);
    table.insert(3, vec![Record::new(3, vec![3])]);

    let reverse_prime_vec: Vec<u64> = PRIME_NUMBER.to_vec().into_iter().rev().collect();

    println!(
        "{:?}",
        max_product(35, &mut table, &reverse_prime_vec).unwrap()[0]
    );

    println!(
        "{:?}",
        max_product(500, &mut table, &reverse_prime_vec).unwrap()[0]
    );

    println!(
        "{:?}",
        max_product(1000, &mut table, &reverse_prime_vec).unwrap()[0]
    );
}
