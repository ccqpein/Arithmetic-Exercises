pub fn addition(num1: String, num2: String) -> String {
    let mut result = vec![];
    let num1_ = (0..num1.len())
        .into_iter()
        .map(|ind| &num1[ind..ind + 1])
        .map(|s| s.parse().unwrap())
        .rev()
        .collect::<Vec<u8>>();

    let num2_ = (0..num2.len())
        .into_iter()
        .map(|ind| &num2[ind..ind + 1])
        .map(|s| s.parse().unwrap())
        .rev()
        .collect::<Vec<u8>>();

    let mut a = 0;

    addition_(&num1_, &num2_, &mut a, &mut result);
    result
        .into_iter()
        .rev()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn addition_(num1: &[u8], num2: &[u8], a: &mut u8, result: &mut Vec<u8>) {
    match (num1, num2) {
        (e @ [], [n, tail @ ..]) | ([n, tail @ ..], e @ []) => {
            let this = *n + *a;
            if this >= 10 {
                result.push(this % 10);
                *a = 1;
                addition_(e, tail, a, result)
            } else {
                result.push(this);
                result.extend_from_slice(tail);
                return;
            }
        }
        ([n1, tail1 @ ..], [n2, tail2 @ ..]) => {
            let this = *n1 + *n2 + *a;
            if this >= 10 {
                result.push(this % 10);
                *a = 1;
                addition_(tail1, tail2, a, result)
            } else {
                result.push(this);
                *a = 0;
                addition_(tail1, tail2, a, result)
            }
        }
        ([], []) => {
            if *a != 0 {
                result.push(*a);
            }
            return;
        }
    }
}

fn main() {
    assert_eq!(addition("123".to_string(), "456".to_string()), "579");
    assert_eq!(addition("1239".to_string(), "456".to_string()), "1695");
    assert_eq!(addition("999".to_string(), "9".to_string()), "1008");
    assert_eq!(
        addition(
            "1000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            "9".to_string()
        ),
        "1000000000000000000000000000000000000000000000000000000000000000000000000000000009"
    );
}
