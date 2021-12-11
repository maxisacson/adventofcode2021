use crate::utils::{day, close, part, read_lines_to_ints, answer};

pub fn run() {
    day(11);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let mut input = read_lines_to_ints("data/day11.txt");

    let rows = input.len();
    let cols = input[0].len();

    let mut result = 0;
    for _ in 0..100 {
        for i in 0..rows {
            for j in 0..cols {
                input[i][j] += 1;
            }
        }

        let mut flashed = vec![vec![false; input[0].len()]; input.len()];
        for i in 0..rows {
            for j in 0..cols {
                result += resolve_flashes(i, j, &mut flashed, &mut input);
            }
        }
    }

    answer(result);
}

fn part2() {
    part(2);

    let mut input = read_lines_to_ints("data/day11.txt");

    let rows = input.len();
    let cols = input[0].len();

    let mut result = 0;
    for step in 0.. {
        for i in 0..rows {
            for j in 0..cols {
                input[i][j] += 1;
            }
        }

        let mut count = 0;
        let mut flashed = vec![vec![false; input[0].len()]; input.len()];
        for i in 0..rows {
            for j in 0..cols {
                count += resolve_flashes(i, j, &mut flashed, &mut input);
            }
        }

        if count == (rows * cols) as u64 {
            result = step+1;
            break;
        }
    }

    answer(result);
}

fn resolve_flashes(i: usize, j: usize, flashed: &mut Vec<Vec<bool>>, input: &mut Vec<Vec<i32>>) -> u64 {
    if flashed[i][j] || input[i][j] <= 9 {
        return 0;
    }

    input[i][j] = 0;
    flashed[i][j] = true;

    let rows = input.len() as i64;
    let cols = input[0].len() as i64;

    let mut count = 1;
    for di in -1..=1 {
        for dj in -1..=1 {
            let i = (i as i64) + di;
            let j = (j as i64) + dj;
            if i >= 0 && i < rows && j >= 0 && j < cols {
                let i = i as usize;
                let j = j as usize;
                if !flashed[i][j] {
                    input[i][j] += 1;
                    count += resolve_flashes(i, j, flashed, input);
                }
            }
        }
    }

    return count;
}
