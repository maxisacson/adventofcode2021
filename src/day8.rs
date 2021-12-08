use crate::utils::{day, close, part, read_lines_to_vec, sort_chars, answer};
use std::collections::HashMap;

pub fn run() {
    day(8);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day8.txt");

    let mut result = 0;
    for line in input {
        let split = line.split('|');
        let lenghts: Vec<i32> = split.skip(1).next().unwrap()
                                          .split_whitespace().map(|x| x.len() as i32).collect();

        result += lenghts.iter().filter(|&&x| x == 2 || x == 4 || x == 3 || x == 7).count();
    }

    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day8.txt");

    let mut result = 0;
    for line in input {
        // Find the 1, 4, 7, 8
        let mut iter = line.split('|');
        let digits: Vec<&str> = iter.next().unwrap().split_whitespace().collect();
        let mut table = Vec::new();
        for digit in &digits {
            match digit.len() {
                2 => table.push((digit, 1)),
                4 => table.push((digit, 4)),
                3 => table.push((digit, 7)),
                7 => table.push((digit, 8)),
                _ => ()
            };
        }

        table.sort_by(|a, b| b.1.cmp(&a.1));

        let mut map = HashMap::new();

        // Determine each digit by how many segments matches with the 4 known ones
        // where ID = X1 X2 X3 X4
        //             |  |  |  |--- matches with 8
        //             |  |  |--- matches with 7
        //             |  |--- matches with 4
        //             |--- matches with 1
        //
        for digit in &digits {
            let mut id = 0;
            for (i, (key, _)) in table.iter().enumerate() {
                let mut matches = 0;
                 for c in digit.chars() {
                    if key.contains(c) {
                        matches += 1;
                    }
                }
                id += matches * 10u32.pow(i as u32);
            }
            let digit = sort_chars(digit);
            match id {
                2336 => map.insert(digit, 0),
                2222 => map.insert(digit, 1),
                1225 => map.insert(digit, 2),
                2335 => map.insert(digit, 3),
                2424 => map.insert(digit, 4),
                1325 => map.insert(digit, 5),
                1326 => map.insert(digit, 6),
                2233 => map.insert(digit, 7),
                2437 => map.insert(digit, 8),
                2436 => map.insert(digit, 9),
                _ => None,
            };
        }

        // Grab the digits to decode
        let digits: Vec<&str> = iter.next().unwrap().split_whitespace().collect();
        let mut number = 0;
        for (i, digit) in digits.iter().rev().enumerate() {
            let digit = map[&sort_chars(digit)];
            number += digit * 10u32.pow(i as u32);
        }

        result += number;
    }

    answer(result);
}
