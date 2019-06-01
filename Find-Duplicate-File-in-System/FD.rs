pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut record: HashMap<String, Vec<String>> = HashMap::new();

    for a in paths {
        let mut split = a.split_ascii_whitespace();
        //Or let mut split = a.split_whitespace();
        let this_path = split.next().unwrap();

        for file in split {
            //println!("{:?}", file);
            let content = file.split(|x| x == '(' || x == ')').collect::<Vec<&str>>();
            //println!("{:?}", content);
            let v = record.entry(String::from(content[1])).or_insert(vec![]);
            v.push(String::from(this_path) + "/" + content[0]);
        }
    }

    let mut result = vec![];
    for v in record.values() {
        if v.len() > 1 {
            result.push(v.clone());
        }
    }

    result
}

fn main() {
    use std::time::Instant;
    let a = Instant::now();
    find_duplicate(vec![
        String::from("root/a 1.txt(abcd) 2.txt(efgh)"),
        String::from("root/c 3.txt(abcd)"),
        String::from("root/c/d 4.txt(efgh)"),
        String::from("root 4.txt(efgh)"),
    ]);
    let b = a.elapsed();
    println!("{}", b.as_micros());

    println!(
        "{:?}",
        find_duplicate(vec![
            String::from("root/a 1.txt(abcd) 2.txt(efgh)"),
            String::from("root/c 3.txt(abcd)"),
            String::from("root/c/d 4.txt(efgh)"),
            String::from("root 4.txt(efgh)")
        ])
    )
}
