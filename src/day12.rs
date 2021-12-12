use std::collections::HashMap;

use crate::utils::{day, close, part, read_lines_to_vec, answer};

pub fn run() {
    day(12);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day12.txt");

    let mut connections = Vec::new();
    for mut split in input.iter().map(|x| x.split("-")) {
        connections.push((split.next().unwrap().to_string(), split.next().unwrap().to_string()));
    }

    let mut map: HashMap<&str, Vec<String>> = HashMap::new();
    for conn in &connections {
        {
            let node0 = map.entry(&conn.0).or_insert(Vec::new());
            node0.push(conn.1.clone());
        }

        {
            let node1 = map.entry(&conn.1).or_insert(Vec::new());
            node1.push(conn.0.clone());
        }

    }

    let paths = find_paths("start", &map, Vec::new(), HashMap::new());
    let paths = paths.iter().filter(|path| path.last().unwrap() == "end");

    let result = paths.count();

    assert_eq!(result, 5958);
    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day12.txt");

    let mut connections = Vec::new();
    for mut split in input.iter().map(|x| x.split("-")) {
        connections.push((split.next().unwrap().to_string(), split.next().unwrap().to_string()));
    }

    let mut map: HashMap<&str, Vec<String>> = HashMap::new();
    for conn in &connections {
        {
            let node0 = map.entry(&conn.0).or_insert(Vec::new());
            node0.push(conn.1.clone());
        }

        {
            let node1 = map.entry(&conn.1).or_insert(Vec::new());
            node1.push(conn.0.clone());
        }

    }

    let paths = find_paths2("start", &map, Vec::new(), HashMap::new(), 0);
    let paths = paths.iter().filter(|path| path.last().unwrap() == "end");

    let result = paths.count();

    assert_eq!(result, 150426);
    answer(result);
}

fn find_paths(current: &str, nodes: &HashMap<&str, Vec<String>>, mut path: Vec<String>, mut visited: HashMap<String, bool>) -> Vec<Vec<String>> {
    if current.chars().all(char::is_lowercase) {
        if visited.get(current) == Some(&true) {
            return vec![path];
        }
        visited.insert(current.to_string(), true);
    }

    path.push(current.to_string());

    if current == "end" {
        return vec![path];
    }

    let mut paths: Vec<Vec<String>> = Vec::new();
    for node in &nodes[current] {
        let found_paths = find_paths(&node, nodes, path.clone(), visited.clone());
        paths.extend(found_paths);
    }

    return paths;
}

fn find_paths2(current: &str, nodes: &HashMap<&str, Vec<String>>, mut path: Vec<String>, mut visited: HashMap<String, i32>, mut count: usize) -> Vec<Vec<String>> {
    if current.chars().all(char::is_lowercase) {
        if !visited.contains_key(current) {
            // OK -- node not visited
            visited.insert(current.to_string(), 1);
            count += 1
        } else {
            if current == "start" {
                // Only visit start once
                return vec![path];
            }
            // Count how many nodes are visited
            if count == visited.len() {
                // OK -- Visited count equal to number of visited nodes => each node visited once
                visited.insert(current.to_string(), visited[current] + 1);
                count += 1;
            } else {
                // One node visited more than once -- return
                return vec![path];
            }
        }
    }

    path.push(current.to_string());

    if current == "end" {
        return vec![path];
    }

    let mut paths: Vec<Vec<String>> = Vec::new();
    for node in &nodes[current] {
        let found_paths = find_paths2(&node, nodes, path.clone(), visited.clone(), count);
        paths.extend(found_paths);
    }

    return paths;
}
