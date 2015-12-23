
use std::io::prelude::*;
use std::fs;

fn main() {
    let mut s: String = String::new();
    let mut f = fs::File::open("input")
        .ok()
        .expect("Can't find file \"input\"");
    f.read_to_string(&mut s).ok().expect("can't read string");
    println!("{}", s);
}
