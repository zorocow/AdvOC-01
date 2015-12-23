
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


const FILENAME: &'static str = "input_d2";

fn calc_areas(s: &[u32; 3]) -> [u32; 3] {
    [
        s[0] * s[1],
        s[1] * s[2],
        s[2] * s[0],
    ]
}

fn calc_slack(s: &[u32; 3]) -> u32 {
    calc_areas(s).iter().fold(u32::max_value(), |acc, &n| match n < acc {
        true => n,
        false => acc,
    })
}

fn calc_total_area(s: &[u32; 3]) -> u32 {
    calc_areas(s).iter().fold(0, |acc, &area| 2 * area + acc)
}

fn main() {
    let f = match File::open(FILENAME) {
        Err(e) => panic!("Can't open file \"{}\": {}", FILENAME, e),
        Ok(f) => f,
    };
    let reader = BufReader::new(f);

    let mut area = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let raw = line.split('x');
        let sizes: Vec<_> = raw.collect();
        assert!(sizes.len() == 3);
        let sizes: [u32; 3] = [sizes[0].parse().unwrap(),
                               sizes[1].parse().unwrap(),
                               sizes[2].parse().unwrap(),];
        area += calc_slack(&sizes) + calc_total_area(&sizes);
    }
    println!("The elves need {} square feet of wrapping paper", area);
}
