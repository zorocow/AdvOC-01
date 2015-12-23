
use std::io::prelude::*;
use std::fs;

fn main() {
    let mut s: String = String::new();
    let mut f = match fs::File::open("input") {
        Err(e) => panic!("Can't open file \"{}\": {}", "input", e),
        Ok(f) => f,
    };
    f.read_to_string(&mut s).ok().expect("can't read string");
    let floor = s.as_bytes().iter().fold(0i64, |acc: i64, &c| match c {
        b'(' => acc+1,
        b')' => acc-1,
        _ => acc,
    });
    println!("Santa should go to floor {}", floor);
}
