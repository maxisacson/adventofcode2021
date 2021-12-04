use crate::utils;

pub fn run() {
    utils::day(2);
    part1();
    part2();
    utils::close();
}

fn part1() {
    utils::part(1);

    let mut horizontal = 0;
    let mut depth = 0;

    if let Ok(lines) = utils::read_lines("data/day2.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                let mut split = line.split_whitespace();
                match split.next() {
                    Some("forward") => {
                        let x = split.next().unwrap().parse::<i32>().unwrap();
                        horizontal += x;
                    },
                    Some("down") => {
                        let x = split.next().unwrap().parse::<i32>().unwrap();
                        depth += x;
                    },
                    Some("up") => {
                        let x = split.next().unwrap().parse::<i32>().unwrap();
                        depth -= x;
                    },
                    Some(&_) => panic!(),
                    None => panic!(),
                }
            }
        }
    }

    utils::answer(horizontal * depth);
}

fn part2 () {
    utils::part(2);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;


    if let Ok(lines) = utils::read_lines("data/day2.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                let mut split = line.split_whitespace();
                match split.next() {
                    Some("forward") => {
                        let x = split.next().unwrap().parse::<i32>().unwrap();
                        horizontal += x;
                        depth += aim * x;
                    },
                    Some("down") => {
                        let x = split.next().unwrap().parse::<i32>().unwrap();
                        aim += x;
                    },
                    Some("up") => {
                        let x = split.next().unwrap().parse::<i32>().unwrap();
                        aim -= x;
                    },
                    Some(&_) => panic!(),
                    None => panic!(),
                }
            }
        }
    }

    utils::answer(horizontal * depth);
}
