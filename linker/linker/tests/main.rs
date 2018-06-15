extern crate linker;

use linker::{Content,SingleLine};

#[test]
#[ignore]
fn test_num_change() {
    let a = linker::read_linker_line(String::from("5 R 1004  I 5678  E 2000  R 8002  E 7001"));
    println!("{:?}", a.content.unwrap().num_change());
}

#[test]
#[ignore]
fn read_linker_line_test() {
    println!(
        "{:?}",
        linker::read_linker_line(String::from("5 R 1004  I 5678  E 2000  R 8002  E 7001"))
    );

    println!("{:?}", linker::read_linker_line(String::from("2 xy z")));

    println!("{:?}", linker::read_linker_line(String::from("0")));

    println!("{:?}", linker::read_linker_line(String::from("2 xy 2")));
}

#[test]
fn test_add_line_vec() {
    let mut r: Vec<SingleLine> = vec![];
    linker::add_line_vec(
        &mut r,
        linker::read_linker_line(String::from("5 R 1004  I 5678  E 2000  R 8002  E 7001")),
    );
    linker::add_line_vec(&mut r, linker::read_linker_line(String::from("2 xy z")));
    linker::add_line_vec(&mut r, linker::read_linker_line(String::from("0")));
    linker::add_line_vec(&mut r, linker::read_linker_line(String::from("2 xy 2")));
    println!("{:?}", r);
}
