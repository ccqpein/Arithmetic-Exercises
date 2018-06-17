use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;
extern crate regex;

//Maybe do not need regex
/*lazy_static! {
    static ref line_re: Regex = Regex::new(r"^\d ").unwrap();
    static ref symbols_re: Regex = Regex::new(r"").unwrap();
    static ref addres_re: Regex = Regex::new(r"").unwrap();
}*/

#[derive(Debug, Clone)]
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
    pub fn num_change(&self) -> Result<Vec<(String, i32)>, ()> {
        let mut result: Vec<(String, i32)> = vec![];
        match self {
            Content::DefSym(l) | Content::Addrs(l) => {
                let (_, l2) = l.split_at(1);
                for (i, (x, y)) in l.iter().zip(l2.iter()).enumerate() {
                    if i % 2 == 0 {
                        result.push((x.clone(), y.parse::<i32>().unwrap()));
                    }
                }
                return Ok(result);
            }
            _ => return Err(()),
        }
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
        if cut_str.len() >= 3 {
            if let Ok(_) = cut_str[2].parse::<i32>() {
                return SingleLine {
                    num: num,
                    content: Some(Content::DefSym(
                        cut_str.split_off(1).iter().map(|s| s.to_string()).collect(),
                    )),
                };
            }
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

//:= need clean mind, I forget requirments actually.
pub fn create_sym_table(line_vec: &Vec<SingleLine>) -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();
    let mut which_use_before_define: HashSet<String> = HashSet::new();
    let mut mem_jump = 0;

    for l in line_vec {
        //println!("{}", mem_jump);
        if let Some(ref c) = l.content {
            match c {
                Content::DefSym(_) => if let Ok(cc) = c.num_change() {
                    for (ind, c_t) in cc.iter().enumerate() {
                        if which_use_before_define.contains(&c_t.0) {
                            result.insert(c_t.0.to_string(), mem_jump + ind as i32);
                        } else {
                            result.insert(c_t.0.to_string(), c_t.1);
                        }
                    }
                    continue;
                },

                Content::Symbols(sl) => {
                    for s in sl {
                        if !result.contains_key(s) {
                            which_use_before_define.insert(s.to_string());
                        }
                    }
                }
                _ => (),
            }
            mem_jump += l.num;
        }
    }

    result
}
