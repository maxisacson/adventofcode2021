use crate::utils;

pub fn run() {
    utils::day(4);
    part1();
    part2();
    utils::close();
}

fn part1() {
    utils::part(1);

    let mut input = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day4.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                input.push(line);
            }
        }
    }

    let numbers: Vec<i32> = input[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Vec<i32>> = Vec::new();
    let mut sums = Vec::new();

    let mut tmp = Vec::new();
    for i in 1..input.len()+1 {
        if i == input.len() || input[i].trim().is_empty() {
            if !tmp.is_empty() {
                let sum: i32 = tmp.iter().sum();
                sums.push(sum);
                boards.push(tmp.to_vec());
            }
            tmp.clear();
        } else {
            let mut tmp2 = input[i].split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            tmp.append(&mut tmp2);
        }
    }

    let mut winning_number = -1;
    let mut winning_board = -1;
    let mut marked: Vec<Vec<i32>> = vec![vec![0; 10]; boards.len()];
    'outer: for n in numbers {
        for (i, board) in boards.iter().enumerate() {
            for (j, &value) in board.iter().enumerate() {
                if value == n {
                    let row: usize = j / 5;
                    let col: usize = j % 5;
                    marked[i][row] += 1;
                    marked[i][5 + col] += 1;
                    sums[i] -= value;
                }
            }

            for &count in &marked[i] {
                if count == 5 {
                    winning_number = n;
                    winning_board = i as i32;
                    break 'outer;
                }
            }
        }
    }

    let winning_sum = sums[winning_board as usize];

    utils::answer(winning_number * winning_sum);
}

fn part2() {
    utils::part(2);

    let mut input = Vec::new();
    if let Ok(lines) = utils::read_lines("data/day4.txt") {
        for line_ in lines {
            if let Ok(line) = line_ {
                input.push(line);
            }
        }
    }

    let numbers: Vec<i32> = input[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Vec<i32>> = Vec::new();
    let mut sums = Vec::new();

    let mut tmp = Vec::new();
    for i in 1..input.len()+1 {
        if i == input.len() || input[i].trim().is_empty() {
            if !tmp.is_empty() {
                let sum: i32 = tmp.iter().sum();
                sums.push(sum);
                boards.push(tmp.to_vec());
            }
            tmp.clear();
        } else {
            let mut tmp2 = input[i].split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            tmp.append(&mut tmp2);
        }
    }

    let mut last_number = -1;
    let mut last_board = -1;
    let mut num_winners = 0;
    let mut winners = vec![0; boards.len()];
    let mut marked: Vec<Vec<i32>> = vec![vec![0; 10]; boards.len()];
    'outer: for n in numbers {
        for (i, board) in boards.iter().enumerate() {
            if winners[i] == 1 {
                continue;
            }

            for (j, &value) in board.iter().enumerate() {
                if value == n {
                    let row: usize = j / 5;
                    let col: usize = j % 5;
                    marked[i][row] += 1;
                    marked[i][5 + col] += 1;
                    sums[i] -= value;
                }
            }

            for &count in &marked[i] {
                if count == 5 {
                    num_winners += 1;
                    winners[i] = 1;
                    if num_winners == boards.len() {
                        last_number = n;
                        last_board = i as i32;
                        break 'outer;
                    }
                    break;
                }
            }
        }
    }

    let last_sum = sums[last_board as usize];

    utils::answer(last_number * last_sum);
}
