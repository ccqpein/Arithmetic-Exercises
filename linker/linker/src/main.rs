use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;
extern crate regex;

//Maybe do not need regex
lazy_static! {
    static ref line_re: Regex = Regex::new(r"^\d ").unwrap();
    static ref symbols_re: Regex = Regex::new(r"").unwrap();
    static ref addres_re: Regex = Regex::new(r"").unwrap();
}

#[derive(Debug)]
enum Content {
    Symbols(Vec<String>),
    Addrs(Vec<String>),
}

#[derive(Debug)]
struct SingleLine {
    num: i32,
    content: Option<Content>,
}

fn read_linker_line(li: String) -> SingleLine {
    let mut cut_str: Vec<&str> = li.split_whitespace().collect();
    let num = cut_str[0].parse::<i32>().unwrap();

    if cut_str[1] != "A" && cut_str[1] != "E" && cut_str[1] != "R" && cut_str[1] != "I" {
        return SingleLine {
            num: num,
            content: Some(Content::Symbols(
                cut_str.split_off(1).iter().map(|s| s.to_string()).collect(),
            )),
        };
    } else {
        return SingleLine {
            num: num,
            content: Some(Content::Addrs(
                cut_str.split_off(1).iter().map(|s| s.to_string()).collect(),
            )),
        };
    }

    return SingleLine {
        num: num,
        content: None,
    };
}

#[test]
fn regex_test() {
    let mut fields: Vec<&str> = line_re
        .split("5 R 1004  I 5678  E 2000  R 8002  E 7001")
        .collect();
    //println!("{:?}", fields);

    fields = line_re
        .split("5 R 1004  I 5678  E 2000  R 8002  E 7001")
        .collect();
}

#[test]
fn read_linker_line_test() {
    println!(
        "{:?}",
        read_linker_line(String::from("5 R 1004  I 5678  E 2000  R 8002  E 7001"))
    );

    println!("{:?}", read_linker_line(String::from("2 xy z")));
}

fn main() {
    let fname = env::args().nth(1).unwrap();
    let mut f = File::open(fname).expect("cannot find file");
    let mut file = BufReader::new(&f);
}
