pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
    let mut text_vec = text.split(' ');

    let mut result: Vec<String> = vec![];

    let mut first_p = text_vec.next();
    let mut second_p = text_vec.next();
    let mut third_p = text_vec.next();

    while let Some(third) = third_p {
        match (first_p, second_p) {
            (Some(first_v), Some(second_v)) => {
                if first_v == first && second_v == second {
                    result.push(String::from(third));
                }
            }
            _ => {
                break;
            }
        }

        first_p = second_p;
        second_p = third_p;
        third_p = text_vec.next();
    }

    result
}

fn main() {
    let testcase = String::from("alice is a good girl she is a good student");
    println!(
        "{:?}",
        find_ocurrences(testcase, String::from("a"), String::from("good"))
    );
}
