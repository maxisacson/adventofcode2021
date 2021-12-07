#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_vec<P>(filename: P) -> Vec<String>
where P: AsRef<Path> {
    io::BufReader::new(File::open(filename)
                    .expect("Could not read file"))
                    .lines().map(|line| line.unwrap()).collect()
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

pub fn min_max<T>(slice: &[T]) -> (T, T)
where T: std::cmp::PartialOrd+Copy {
    let mut max = slice[0];
    let mut min = slice[0];

    for &x in slice {
        max = if x > max { x } else { max };
        min = if x < min { x} else { min };
    }

    (min, max)
}
