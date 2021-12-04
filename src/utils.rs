use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day(day: i32) {
    println!("++++++++++ Day {} ++++++++++", day);
}

pub fn part(part: i32) {
    println!("--- Part {} ---", part);
}

pub fn close() {
    println!("");
}

pub fn answer<T>(answer: T)
where T: std::fmt::Display, {
    println!("Answer: {}", answer);
}
