
use std::io::prelude::*;
use std::fs;

const FILENAME: &'static str = "input_d1";
fn main() {
    let mut s: String = String::new();
    let mut f = match fs::File::open(FILENAME) {
        Err(e) => panic!("Can't open file \"{}\": {}", FILENAME, e),
        Ok(f) => f,
    };
    f.read_to_string(&mut s).ok().expect("can't read string");
    let floor = s.chars().fold(0i64, |acc: i64, c| match c {
        '(' => acc+1,
        ')' => acc-1,
        _ => acc,
    });
    println!("Santa should go to floor {}", floor);
}
