use crate::utils::{day, close, part, read_lines_to_vec, answer};
use std::collections::HashMap;

pub fn run() {
    day(10);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day10.txt");

    let points = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let opening = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    let mut result = 0;
    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.pop() != Some(opening[&c]) {
                        result += points[&c];
                        break;
                    }
                },
                _ => {},
            }
        }
    }

    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day10.txt");

    // the scores in the problem are given by the closing variant of the charactes,
    // but this doesn't matter as we know that a closing ) should go after an orhaned (
    // in the stack
    let points = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let opening = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    let mut scores = Vec::new();
    'outer: for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.pop() != Some(opening[&c]) {
                        continue 'outer; // Skip corrupt lines
                    }
                },
                _ => {},
            }
        }

        // This should now be an incomplete line
        let mut score: u64 = 0;
        for c in stack.iter().rev() {
            score *= 5;
            score += points[c];
        }
        scores.push(score);
    }
    scores.sort();

    let result = scores[scores.len()/2];
    answer(result);
}
