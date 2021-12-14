use crate::utils::{day, close, part, read_lines_to_vec, answer};
use std::collections::HashMap;

pub fn run() {
    day(14);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day14.txt");
    let mut polymer = input[0].clone();

    let mut rules = HashMap::new();
    for rule in input.iter().skip(2) {
        let mut split = rule.split(" -> ");
        let pair = split.next().unwrap();
        let insert = split.next().unwrap().chars().next().unwrap();
        rules.insert(pair, insert);
    }


    let mut counts = HashMap::new();
    for c in polymer.chars() {
        counts.insert(c, 1);
    }


    for _ in 0..10 {
        let mut offset = 0;
        let length = polymer.len();
        for i in 0..length-1 {
            let j = i + offset;
            let pair = &polymer[j..j+2];
            match rules.get(pair) {
                Some(&insert) => {
                    polymer.insert(j+1, insert);
                    let count = counts.entry(insert).or_insert(0);
                    *count += 1;
                    offset += 1;
                },
                _ => {},
            }
        }
    }

    let mut counts: Vec<(char, i32)> = counts.drain().collect();
    counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
    let most = counts.first().unwrap().1;
    let least = counts.last().unwrap().1;

    let result = most - least;

    assert_eq!(result, 2584);
    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day14.txt");
    let polymer = &input[0];

    let mut rules = HashMap::new();
    for rule in input.iter().skip(2) {
        let mut split = rule.split(" -> ");
        let pair = split.next().unwrap();
        let insert = split.next().unwrap().chars().next().unwrap();
        rules.insert(pair, insert);
    }


    let mut counts = HashMap::new();
    for c in polymer.chars() {
        counts.insert(c, 1usize);
    }


    let mut cache: HashMap<(String, i32), HashMap<char, usize>> = HashMap::new();
    let length = polymer.len();
    for i in 0..length-1 {
        let pair = &polymer[i..i+2];
        expand(pair, &rules, &mut counts, 0, 40, &mut cache);
    }

    let mut counts: Vec<(char, usize)> = counts.drain().collect();
    counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
    let most = counts.first().unwrap().1;
    let least = counts.last().unwrap().1;

    let result = most - least;

    assert_eq!(result, 3816397135460);
    answer(result);
}

fn expand(pair: &str, rules: &HashMap<&str, char>, counts: &mut HashMap<char, usize>, step: i32,
          step_limit: i32, cache: &mut HashMap<(String, i32), HashMap<char, usize>>) {
    if step >= step_limit {
        return;
    }

    match cache.get(&(pair.to_string(), step_limit - step)) {
        Some(partial_counts) => {
            for (key, val) in partial_counts {
                let count = counts.entry(*key).or_insert(0);
                *count += *val;
            }
            return;
        },
        _ => {}
    }

    let counts_saved = counts.clone();

    let c = rules[pair];
    let count = counts.entry(c).or_insert(0);
    *count += 1;

    let a = format!("{}{}", &pair[..1], c);
    let b = format!("{}{}", c, &pair[1..]);

    expand(&a, rules, counts, step+1, step_limit, cache);
    expand(&b, rules, counts, step+1, step_limit, cache);
    let steps_taken = step_limit - step;

    let mut partial_counts = HashMap::new();
    for (key, val) in counts {
        let &old_count = counts_saved.get(key).unwrap_or(&0);
        let partial_count = *val - old_count;
        partial_counts.insert(*key, partial_count);
    }
    cache.insert((pair.to_string(), steps_taken), partial_counts);
}
