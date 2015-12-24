

use std::io::prelude::*;
use std::fs;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(i32, i32);

impl Pos {

    fn move_left(&self) -> Pos {
        Pos(self.0-1, self.1)
    }

    fn move_right(&self) -> Pos {
        Pos(self.0+1, self.1)
    }

    fn move_up(&self) -> Pos {
        Pos(self.0, self.1+1)
    }

    fn move_down(&self) -> Pos {
        Pos(self.0, self.1-1)
    }
}

const FILENAME: &'static str = "input_d3";

fn main() {
    let mut s: String = String::new();
    let mut f = match fs::File::open(FILENAME) {
        Err(e) => panic!("Can't open file \"{}\": {}", FILENAME, e),
        Ok(f) => f,
    };
    f.read_to_string(&mut s).ok().expect("can't read string");
    let mut visited = Vec::<Pos>::new();

    santa_run(&s, &mut visited);
    visited.sort();
    visited.dedup();
    println!("Solo Santa visited {} unique houses", visited.len());

    visited.clear();

    let inst = s.clone();
    let inst_1: String = inst.char_indices()
        .filter_map(|x| {
            match (x.0 % 2, x.1) {
                (0, c) => Some(c),
                _ => None,
            }})
        .collect();

    let inst_2: String = inst.char_indices()
        .filter_map(|x| {
            match (x.0 % 2, x.1) {
                (1, c) => Some(c),
                _ => None,
            }})
        .collect();

    santa_run(&inst_1, &mut visited);
    santa_run(&inst_2, &mut visited);

    visited.sort();
    visited.dedup();
    println!("Team Santa visited {} unique houses", visited.len());
}

fn santa_run(inst: &str, visited: &mut Vec<Pos>) {
    let mut santa = Pos(0, 0);

    visited.push(santa);
    for c in inst.chars() {
        santa = match c {
            '<' => santa.move_left(),
            '>' => santa.move_right(),
            '^' => santa.move_up(),
            'V'|'v' => santa.move_down(),
            _ => santa,
        };
        visited.push(santa);
    }
}
