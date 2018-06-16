use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

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
pub enum Content {
    DefSym(Vec<String>),
    Symbols(Vec<String>),
    Addrs(Vec<String>),
}

#[derive(Debug)]
pub struct SingleLine {
    pub num: i32,
    pub content: Option<Content>,
}

impl Content {
    pub fn num_change(&mut self) -> Vec<(String, i32)> {
        let mut result: Vec<(String, i32)> = vec![];
        match self {
            Content::DefSym(l) | Content::Addrs(l) => {
                let (_, l2) = l.split_at(1);
                for (i, (x, y)) in l.iter().zip(l2.iter()).enumerate() {
                    if i % 2 == 0 {
                        result.push((x.clone(), y.parse::<i32>().unwrap()));
                    }
                }
            }
            _ => (),
        }
        return result;
    }
}

pub fn read_linker_line(li: String) -> SingleLine {
    let mut cut_str: Vec<&str> = li.split_whitespace().collect();
    let num = cut_str[0].parse::<i32>().unwrap();

    if cut_str.len() == 1 {
        return SingleLine {
            num: num,
            content: None,
        };
    }

    if cut_str[1] != "A" && cut_str[1] != "E" && cut_str[1] != "R" && cut_str[1] != "I" {
        if let Ok(_) = cut_str[2].parse::<i32>() {
            return SingleLine {
                num: num,
                content: Some(Content::DefSym(
                    cut_str.split_off(1).iter().map(|s| s.to_string()).collect(),
                )),
            };
        }
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
}


pub fn add_line_vec(ve: &mut Vec<SingleLine>, l: SingleLine) {
    ve.push(l);
}

fn iter_all_lines() -> (HashMap, Vec<(String,i32)>) {
    return HashMap::new(), vec![("a",1)];
}
