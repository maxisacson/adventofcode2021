use crate::utils::{day, close, part, read_lines_to_vec, answer};
use std::collections::HashSet;

pub fn run() {
    day(13);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day13.txt");

    let mut points = Vec::new();
    let mut set = HashSet::new();
    let mut instructions = Vec::new();
    let mut parse_points = true;
    for line in &input {
        if line.is_empty() {
            parse_points = false;
            continue;
        }

        if parse_points {
            let mut split = line.split(",").map(|x| x.parse::<i32>().unwrap());
            let point = (split.next().unwrap(), split.next().unwrap());
            points.push(point);
            set.insert(point);
        } else {
            let mut split = line.split_whitespace().skip(2).next().unwrap().split("=");
            let axis = split.next().unwrap();
            let value = split.next().unwrap().parse::<i32>().unwrap();
            instructions.push((axis, value));
        }
    }

    let instruction = instructions[0];
    let axis = instruction.0;
    let value = instruction.1;

    for point in points {
        fold(&point, axis, value, &mut set);
    }

    let result = set.len();

    assert_eq!(result, 795);
    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day13.txt");

    let mut set = HashSet::new();
    let mut instructions = Vec::new();
    let mut parse_points = true;
    for line in &input {
        if line.is_empty() {
            parse_points = false;
            continue;
        }

        if parse_points {
            let mut split = line.split(",").map(|x| x.parse::<i32>().unwrap());
            let point = (split.next().unwrap(), split.next().unwrap());
            set.insert(point);
        } else {
            let mut split = line.split_whitespace().skip(2).next().unwrap().split("=");
            let axis = split.next().unwrap();
            let value = split.next().unwrap().parse::<i32>().unwrap();
            instructions.push((axis, value));
        }
    }

    let mut width = 0;
    let mut height = 0;
    for instruction in instructions {
        let axis = instruction.0;
        let value = instruction.1;

        let points: Vec<(i32, i32)> = set.iter().cloned().collect();
        for point in &points {
            fold(&point, axis, value, &mut set);
        }
        width = if axis == "x" { value } else { width };
        height = if axis == "y" { value } else { height };
    }

    assert_eq!(set.len(), 88);
    print_grid(width, height, &set);
}

fn print_grid(width: i32, height: i32, set: &HashSet<(i32, i32)>) {
    for j in 0..height {
        for i in 0..width {
            if set.contains(&(i, j)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold(point: &(i32, i32), axis: &str, value: i32, set: &mut HashSet<(i32, i32)>) {
    match axis {
        "x" => {
            if point.0 < value {
                return;
            }

            let x = 2*value - point.0;
            set.remove(&point);
            set.insert((x, point.1));
        },
        "y" => {
            if point.1 < value {
                return;
            }

            let y = 2*value - point.1;
            set.remove(&point);
            set.insert((point.0, y));
        },
        _ => {},
    }
}
