use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use Markov_Chain::*;

fn main() -> Result<()> {
    let mut file = File::open("./test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let v: Vec<String> = contents
        .split(|c| c == ' ' || c == '\n')
        .map(|s| String::from(s))
        .collect();
    dbg!(&v);
    let mut a = WholeArticleMap::new();
    a.read_to_end(v);
    dbg!(&a);

    Ok(())
}
