use crate::utils::{day, close, part, read_lines_to_vec, answer};
use std::{collections::HashSet, collections::BinaryHeap};
use std::cmp::Ordering;

pub fn run() {
    day(15);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day15.txt");

    let rows = input.len() as i32;
    let cols = input[0].len() as i32;
    let mut tiles = Vec::new();


    // Naive Dijkstra's algorithm

    let mut unvisited = HashSet::new();
    for i in 0..rows {
        for j in 0..cols {
            unvisited.insert((j + cols * i) as usize);
            let risk = input[i as usize].chars().skip(j as usize).next()
                .unwrap().to_digit(10).unwrap() as i32;
            tiles.push(Tile{risk, score: i32::MAX, visited: false});
        }
    }

    tiles[0].score = 0;
    let mut current = Guess{index: 0, score: 0};
    loop {
        // For each neighbour
        for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let k = current.index / rows as usize;
            let l = current.index % cols as usize;
            let i = k as i32 + i;
            let j = l as i32 + j;
            if i >= 0 && i < rows && j >= 0 && j < cols {
                let index = (j + i * cols) as usize;
                let neighbour = &tiles[index];
                if neighbour.visited {
                    continue;
                }
                let score = tiles[current.index].score + neighbour.risk;
                if score < neighbour.score {
                    tiles[index].score = score;
                }
            }
        }

        tiles[current.index].visited = true;
        unvisited.remove(&current.index);
        if current.index == (rows * cols - 1) as usize {
            break;
        }

        // This is pretty dumb
        current.score = i32::MAX;
        for &index in &unvisited {
            let score = tiles[index].score;
            if score < current.score {
                current.score = score;
                current.index = index;
            }
        }
    }
    let result = current.score;

    assert_eq!(result, 540);
    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day15.txt");

    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    let mut tiles = Vec::new();


    // Dijkstra's algorithm with binary heap

    for i in 0..5*rows {
        for j in 0..5*cols {
            let n = i / rows;
            let m = j / cols;
            let mut risk = input[(i - n*rows) as usize].chars().skip((j - m*cols) as usize).next()
                .unwrap().to_digit(10).unwrap() as i32;

            risk += n + m;
            risk = (risk - 1) % 9 + 1;
            tiles.push(Tile{risk, score: i32::MAX, visited: false});
        }
    }
    tiles[0].score = 0;

    let rows = 5 * rows;
    let cols = 5 * cols;

    let mut heap = BinaryHeap::new();
    heap.push(Guess{index: 0, score: 0});

    let mut result = 0;
    while let Some(current) = heap.pop() {
        if current.index == (rows*cols-1) as usize {
            result = current.score;
            break;
        }

        if current.score > tiles[current.index].score { continue };

        // For each neighbour
        for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let k = current.index / rows as usize;
            let l = current.index % cols as usize;
            let i = k as i32 + i;
            let j = l as i32 + j;
            if i >= 0 && i < rows && j >= 0 && j < cols {
                let index = (j + i * cols) as usize;
                if tiles[index].visited {
                    continue;
                }
                let score = current.score + tiles[index].risk;
                if score < tiles[index].score {
                    heap.push(Guess{index, score});
                    tiles[index].score = score;
                }
            }
        }
        tiles[current.index].visited = true;
    }

    assert_eq!(result, 2879);
    answer(result);
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    risk: i32,
    score: i32,
    visited: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Guess {
    index: usize,
    score: i32,
}

impl Ord for Guess {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
