use crate::utils;
use std::collections::HashMap;

pub fn run() {
    utils::day(5);
    part1();
    part2();
    utils::close();
}

struct LineSegment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn part1() {
    utils::part(1);

    let input = utils::read_lines_to_vec("data/day5.txt");

    let lines: Vec<LineSegment> = input.iter().map(|l| {
        let mut iter = l.split(|x: char| !x.is_digit(10)).filter(|s| !s.is_empty());
        LineSegment {
            x1: iter.next().unwrap().parse().unwrap(),
            y1: iter.next().unwrap().parse().unwrap(),
            x2: iter.next().unwrap().parse().unwrap(),
            y2: iter.next().unwrap().parse().unwrap(),
        }
    }).collect();

    let mut points = HashMap::new();

    for line in lines {
        if line.x1 != line.x2 && line.y1 != line.y2 {
            continue;
        }

        let dx = if line.x2 > line.x1 { 1 } else if line.x2 < line.x1 { -1 } else { 0 };
        let dy = if line.y2 > line.y1 { 1 } else if line.y2 < line.y1 { -1 } else { 0 };

        let mut x = line.x1;
        let mut y = line.y1;
        while dx*x <= dx*line.x2 && dy*y <= dy*line.y2 {
            let pos = (x, y);
            let count = points.entry(pos).or_insert(0);
            *count += 1;

            x += dx;
            y += dy;
        }
    }

    let answer = points.values().filter(|&&v| v >= 2).count();

    utils::answer(answer);
}

fn part2() {
    utils::part(2);

    let input = utils::read_lines_to_vec("data/day5.txt");

    let lines: Vec<LineSegment> = input.iter().map(|l| {
        let mut iter = l.split(|x: char| !x.is_digit(10)).filter(|s| !s.is_empty());
        LineSegment {
            x1: iter.next().unwrap().parse().unwrap(),
            y1: iter.next().unwrap().parse().unwrap(),
            x2: iter.next().unwrap().parse().unwrap(),
            y2: iter.next().unwrap().parse().unwrap(),
        }
    }).collect();

    let mut points = HashMap::new();

    for line in lines {
        let dx = if line.x2 > line.x1 { 1 } else if line.x2 < line.x1 { -1 } else { 0 };
        let dy = if line.y2 > line.y1 { 1 } else if line.y2 < line.y1 { -1 } else { 0 };

        let mut x = line.x1;
        let mut y = line.y1;
        while dx*x <= dx*line.x2 && dy*y <= dy*line.y2 {
            let pos = (x, y);
            let count = points.entry(pos).or_insert(0);
            *count += 1;

            x += dx;
            y += dy;
        }

    }

    let answer = points.values().filter(|&&v| v >= 2).count();

    utils::answer(answer);
}
