use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let mut f = File::open(fname).expect("cannot find file");
    let mut file = BufReader::new(&f);
}
