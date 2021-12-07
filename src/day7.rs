use crate::utils::{day, close, part, read_lines_to_vec, min_max, answer};

pub fn run() {
    day(7);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day7.txt");
    let input: Vec<i32> = input[0].split(",").map(|x| x.parse().unwrap()).collect();

    let (min, max) = min_max(&input);
    let mut cost = i32::MAX;
    for x in min..=max {
        let current_cost: i32 = input.iter().map(|pos| (pos - x).abs()).sum();
        if current_cost < cost {
            cost = current_cost;
        }
    }

    answer(cost);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day7.txt");
    let input: Vec<i32> = input[0].split(",").map(|x| x.parse().unwrap()).collect();

    let (min, max) = min_max(&input);
    let mut cost = i32::MAX;
    for x in min..=max {
        let current_cost: i32 = input.iter().map(|pos| {
            let steps = (pos - x).abs();
            steps * (steps + 1) / 2
        }).sum();

        if current_cost < cost {
            cost = current_cost;
        }
    }

    answer(cost);
}
