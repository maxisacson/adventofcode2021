use crate::utils;

pub fn run() {
    utils::day(1);
    part1();
    part2();
    utils::close();
}

fn part1() {
    utils::part(1);
    let mut depths = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day1.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                let x = line.parse::<i32>().unwrap();
                depths.push(x);
            }
        }
    }

    let mut count = 0;
    for i in 1..depths.len() {
        let xprev = depths[i-1];
        let x = depths[i];
        count += (x - xprev > 0) as i32;
    }

    utils::answer(count);
}

fn part2 () {
    utils::part(2);

    let mut depths = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day1.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                let x = line.parse::<i32>().unwrap();
                depths.push(x);
            }
        }
    }


    let mut count = 0;
    for i in 3..depths.len() {
        let prev = depths[i-3] + depths[i-2] + depths[i-1];
        let sum = depths[i-2] + depths[i-1] + depths[i];

        count += (sum - prev > 0) as i32;
    }

    utils::answer(count);
}
