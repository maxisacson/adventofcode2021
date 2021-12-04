use crate::utils;

pub fn run() {
    utils::day(3);
    part1();
    part2();
    utils::close();
}

fn part1() {
    utils::part(1);

    let mut report = Vec::new();

    if let Ok(lines) = utils::read_lines("data/day3.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                report.push(line);
            }
        }
    }

    let bit_count = report[0].chars().count();
    let mut gamma = 0b00000;
    for i in 0..bit_count {
        let mut sum = 0;
        for line in &report {
            if let Some(bit) = line.chars().nth(i).unwrap().to_digit(2) {
                sum += if bit == 1 { 1 } else { -1 };
            }
        }
        if sum > 0 {
            gamma |= 1 << (bit_count - 1 - i);
        } else if sum < 0 {
            gamma &= !(1 << (bit_count - 1 - i));
        } else {
            panic!();
        }
    }
    let epsilon = (!gamma) & !(u32::MAX << bit_count);

    utils::answer(gamma * epsilon);
}

fn part2() {
    utils::part(2);

    let mut report = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day3.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                report.push(line);
            }
        }
    }

    let oxygen = isize::from_str_radix(&filter(0, 1, &report), 2).unwrap();
    let scrubber = isize::from_str_radix(&filter(0, -1, &report), 2).unwrap();

    utils::answer(oxygen * scrubber);
}

fn filter(pos: usize, sign: i32, strings: &Vec<String>) -> String {
    if strings.len() == 1 {
        return strings[0].clone();
    }

    if pos == strings[0].chars().count() {
        panic!();
    }

    let mut zeros = Vec::new();
    let mut ones = Vec::new();
    for string in strings {
        let char = string.chars().nth(pos);
        match char {
            Some('0') => zeros.push(string.clone()),
            Some('1') => ones.push(string.clone()),
            Some(_) => panic!(),
            None => panic!()
        }
    }

    let count_zeros = sign * (zeros.len() as i32);
    let count_ones = sign * (ones.len() as i32);

    if count_zeros > count_ones {
        return filter(pos+1, sign, &zeros);
    } else if count_zeros < count_ones {
        return filter(pos+1, sign, &ones);
    } else if sign > 0 {
        return filter(pos+1, sign, &ones);
    } else {
        return filter(pos+1, sign, &zeros);
    }
}





