fn remove_element<T>(a: Vec<T>, k: T) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut result = vec![];
    for i in a {
        if i != k {
            result.push(i);
        }
    }
    result
}

fn main() {
    let test_case = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", remove_element(test_case, 4));
}
