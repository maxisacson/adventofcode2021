use crate::utils::{day, close, part, read_lines_to_vec, answer};

pub fn run() {
    day(9);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day9.txt");

    let input: Vec<Vec<i32>> = input.iter()
        .map(|v| v.chars()
             .map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();

    let mut result = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            if is_low_point(i, j, &input) {
                let risk = 1 + x;
                result += risk;
            }
        }
    }

    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day9.txt");

    let input: Vec<Vec<i32>> = input.iter()
        .map(|v| v.chars()
             .map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();

    // find the low points
    let mut low_points = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if is_low_point(i, j, &input) {
                low_points.push((i, j));
            }
        }
    }

    // find all basins using BFS
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut basins = Vec::new();
    for (i, j) in low_points {
        let current = explore_basin(i, j, &input, &mut visited);
        basins.push(current);
    }
    basins.sort_by(|a, b| b.cmp(a));

    let result = basins[..3].iter().fold(1, |a, &x| a*x);

    answer(result);
}

fn explore_basin(i: usize, j:usize, heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
    if visited[i][j] {
        return 0;
    }
    visited[i][j] = true;

    if heights[i][j] == 9 {
        return 0;
    }

    let mut result = 1;

    let x = heights[i][j];

    let rows = heights.len() as i64;
    let cols = heights[0].len() as i64;
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for d in directions {
        let ii = (i as i64) + d.0;
        let jj = (j as i64) + d.1;
        if ii < rows && ii >= 0 && jj < cols && jj >= 0 {
            let ii = ii as usize;
            let jj = jj as usize;
            if heights[ii][jj] > x {
                result += explore_basin(ii, jj, heights, visited);
            }
        }
    }

    result
}

fn is_low_point(i: usize, j: usize, heights: &Vec<Vec<i32>>) -> bool {
    let rows = heights.len() as i64;
    let cols = heights[0].len() as i64;

    let x = heights[i][j];

    let mut result = true;

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for d in directions {
        let ii = (i as i64) + d.0;
        let jj = (j as i64) + d.1;
        if ii < rows && ii >= 0 && jj < cols && jj >= 0 {
            let ii = ii as usize;
            let jj = jj as usize;
            result &= heights[ii][jj] > x;
        }
    }

    result
}

