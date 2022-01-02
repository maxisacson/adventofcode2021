use crate::utils::{day, close, part, read_lines_to_vec, answer};
use std::{fmt, ops::{self, Add}, process::exit};

pub fn run() {
    day(18);
    part1();
    // part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/sample.txt");

    let mut sum = input[0].clone();

    for number in input.iter().skip(1) {
        println!("  {}", sum);
        println!("+ {}", number);
        sum = add_numbers(&sum, &number);
        reduce_number(&mut sum);
        println!("= {}", sum);
        println!();
        // break;
    }


    // let number1 = &input[0];
    // let number2 = &input[1];
    //
    // println!("{}", number1);
    // println!("{}", number2);
    // let mut number = add_numbers(number1, number2);
    // println!("addition: {}", number);
    // reduce_number(&mut number);
    // println!("reduced:  {}", number);
    // explode_number(&mut number);
    // println!("explode:  {}", number);
    // explode_number(&mut number);
    // println!("explode:  {}", number);
    // split_number(&mut number);
    // println!("split:    {}", number);
    // split_number(&mut number);
    // println!("split:    {}", number);
    // explode_number(&mut number);
    // println!("explode:  {}", number);
    //
    // explode_number(&mut number);
    // println!("explode:  {}", number);
    // split_number(&mut number);
    // println!("split:    {}", number);

    // let chars: Vec<char> = input[0].chars().collect();
    // if let (NodeType::Node(number), _) = parse_number(&chars, 0) {
    //     println!("{}", number);
    // }

    // let mut p1 = Pair{a: 1, b: 2};
    // let mut p2 = Pair{a: 3, b: 4};
    // let mut q = p1 + p2;
    //
    // p1.a = 7;
    // println!("{}", p1);
    // println!("{}", p2);
    // println!("{}", q);
    //
    // let node1 = Node{a: NodeType::Number(5), b: NodeType::Number(10)};
    // let node2 = Node{a: NodeType::Number(1), b: NodeType::Node(&node1)};
    //
    // println!("{}", node1);
    // println!("{}", node2);

    // let mut result = 0;

    // assert_eq!(result, );
    // answer(result);
}

fn add_numbers(s1: &str, s2: &str) -> String {
    format!("[{},{}]", s1.to_string(), s2.to_string())
}

fn reduce_number(number: &mut String) {
    loop {
        if let Some(n) = find_explodable_number(number) {
            // let i = n.first;
            // let j = n.second;
            // let mut s = vec![' '; i];
            // s.push('f');
            // s.append(&mut vec![' '; j-i-1]);
            // s.push('s');
            //
            // if let Some(left) = n.left {
            //     s[left] = 'l';
            // }
            //
            // if let Some(right) = n.right {
            //     s.append(&mut vec![' '; right-j-1]);
            //     s.push('r');
            // }

            // println!("{}", number);
            // println!("{}", String::from_iter(s));
            explode_number(number, n);

            continue;
        }

        if let Some(i) = find_splitable_number(number) {
            // let mut s = vec![' '; i];
            // s.push('^');
            // println!("{}", number);
            // println!("{}", String::from_iter(s));
            split_number(number, i);

            continue;
        }

        // let expl = explode_number(number);
        // if expl {
        //     println!("exploded: {}", number);
        // } else {
        //     split = split_number(number);
        //     if split {
        //         println!("split:    {}", number);
        //     }
        // }
        // reducible = expl || split;
        break;
    }
}

struct Explodable {
    first: usize,
    second: usize,
    left: Option<usize>,
    right: Option<usize>,
}

fn find_explodable_number(s: &str) -> Option<Explodable> {
    let mut i = 0;
    let mut j;
    let mut k;
    let mut nesting = 0;
    let mut left: Option<usize> = None;
    let mut right: Option<usize> = None;

    let mut chars: Vec<char> = s.chars().collect();
    while nesting <= 4 {
        if i >= chars.len()-3 {
            return None;
        }

        if chars[i].is_digit(10) {
            left = Some(i);
        }

        nesting += (chars[i] == '[') as u32;
        nesting -= (chars[i] == ']') as u32;

        i += 1;
    }

    j = i+1;
    while chars[j] != ',' {
        j += 1;
    }

    k = j+1;
    while chars[k] != ']' {
        k += 1;
    }

    // if let Err(_) = s[i..j].parse::<u32>() {
    //     continue;
    // }
    //
    // if let Err(_) = s[j+1..k].parse::<u32>() {
    //     continue;
    // }
    // println!("{}", s[i..j].to_string());
    // println!("{}", s[j+1..k].to_string());
    //
    // exit(1);
    // }

    for l in k+1..chars.len() {
        if chars[l].is_digit(10) {
            right = Some(l);
            break;
        }
    }

    return Some(Explodable{first: i, second: j+1, left, right});
}

fn explode_number(s: &mut String, n: Explodable) {
    let mut i = n.first;
    let j = n.second;
    let mut k = j + 1;

    let len0 = s.len();

    let chars: Vec<char> = s.chars().collect();
    while chars[k] != ']' {
        k += 1;
    }

    let a: u32 = s[i..j-1].parse().unwrap();
    let b: u32 = s[j..k].parse().unwrap();

    if let Some(l) = n.left {
        let mut m = l + 1;
        while chars[m].is_digit(10) {
            m += 1;
        }
        let mut left: u32 = s[l..m].parse().unwrap();
        left += a;

        *s = format!("{}{}{}", s[0..l].to_string(), left, s[m..].to_string());
    }

    let diff = s.len() - len0;
    i += diff;
    // j += diff;
    k += diff;

    if let Some(mut r) = n.right {
        let mut m = r + 1;
        while chars[m].is_digit(10) {
            m += 1;
        }
        r += diff;
        m += diff;
        // println!("{}:{} {}", r, m, s[r..m].to_string());
        let mut right: u32 = s[r..m].parse().unwrap();
        right += b;

        *s = format!("{}{}{}", s[0..r].to_string(), right, s[m..].to_string());
    }

    *s = format!("{}0{}", s[0..i-1].to_string(), s[k+1..].to_string());

    // println!("{}", s);
}

// fn explode_number(s: &mut String) -> bool {
//     let mut i = 0;
//     let mut j;
//     let mut k;
//     let mut nesting = 0;
//     let mut left: Option<usize> = None;
//     let mut right: Option<usize> = None;
//
//     let mut chars: Vec<char> = s.chars().collect();
//     // while nesting <= 4 {
//     loop {
//         if i >= chars.len()-3 {
//             return false;
//         }
//
//         if chars[i].is_digit(10) {
//             left = Some(i);
//         }
//
//         nesting += (chars[i] == '[') as u32;
//         nesting -= (chars[i] == ']') as u32;
//
//         i += 1;
//
//         if nesting > 4 && chars[i].is_digit(10) {// && chars[i+2].is_digit(10) {
//             j = i+1;
//             while chars[j] != ',' {
//                 j += 1;
//             }
//
//             k = i+1;
//             while chars[k] != ']' {
//                 k += 1;
//             }
//
//             if let Err(_) = s[i..j].parse::<u32>() {
//                 continue;
//             }
//
//             if let Err(_) = s[j+1..k].parse::<u32>() {
//                 continue;
//             }
//             // println!("{}", s[i..j].to_string());
//             // println!("{}", s[j+1..k].to_string());
//             //
//             // exit(1);
//             break;
//         }
//     }
//
//     // let mut j = i+1;
//     // while chars[j] != ',' {
//     //     j += 1;
//     // }
//
//     // let mut k = i+1;
//     // while chars[k] != ']' {
//     //     k += 1;
//     // }
//
//     for l in k+1..chars.len() {
//         if chars[l].is_digit(10) {
//             right = Some(l);
//             break;
//         }
//     }
//
//     // println!("{}", s);
//     // println!("({},{}): {}", i, k, s[i..k].to_string());
//     let a: u32 = s[i..k].split(',').next().unwrap().parse().unwrap();
//     let b: u32 = s[i..k].split(',').skip(1).next().unwrap().parse().unwrap();
//
//     if let Some(left) = left {
//         let x = a + chars[left].to_digit(10).unwrap();
//         *s = format!("{}{}{}", s[0..left].to_string(), x, s[left+1..].to_string());
//         if x >= 10 {
//             if let Some(r) = right {
//                 right = Some(r+1);
//             }
//             i += 1;
//             // j += 1;
//             k += 1;
//             chars = s.chars().collect();
//         }
//     }
//     // println!("s={}", s);
//
//     if let Some(right) = right {
//         // println!("right={}", right);
//         let x = b + chars[right].to_digit(10).unwrap();
//         *s = format!("{}{}{}", s[0..right].to_string(), x, s[right+1..].to_string());
//     }
//     *s = format!("{}0{}", s[0..i-1].to_string(), s[k+1..].to_string());
//
//     return true;
// }

fn find_splitable_number(s: &str) -> Option<usize> {
    let mut i = 0;
    let mut j;

    let chars: Vec<char> = s.chars().collect();
    loop {
        if i >= chars.len()-1 {
            return None;
        };

        if !chars[i].is_digit(10) {
            i += 1;
            continue;
        }

        j = i + 1;
        while chars[j].is_digit(10) {
            j += 1;
        }

        if j > i + 1 {
            return Some(i);
        }

        i = j + 1;
    }
}

fn split_number(s: &mut String, i: usize) {
    let mut j = i + 1;

    let chars: Vec<char> = s.chars().collect();
    while chars[j].is_digit(10) {
        j += 1;
    }

    let x: u32 = s[i..j].parse().unwrap();
    let left = x / 2;
    let right = (x+1) / 2;
    *s = format!("{}[{},{}]{}", s[0..i].to_string(), left, right, s[j..].to_string());
}

// fn split_number(s: &mut String) -> bool {
//     let mut i = 0;
//     let mut j;
//
//     let chars: Vec<char> = s.chars().collect();
//     loop {
//         if i >= chars.len()-1 {
//             return false;
//         };
//
//         if !chars[i].is_digit(10) {
//             i += 1;
//             continue;
//         }
//
//         j = i;
//         while chars[j].is_digit(10) {
//             j += 1;
//         }
//
//         if j > i + 1 {
//             break;
//         }
//
//         i = j;
//     }
//     // println!("{}, {}  ({})", i, j, chars.len());
//
//     let x: u32 = s[i..j].parse().unwrap();
//     let left = x / 2;
//     let right = (x+1) / 2;
//     *s = format!("{}[{},{}]{}", s[0..i].to_string(), left, right, s[j..].to_string());
//
//     return true;
// }

// fn part2() {
//     part(2);
//
//     let input = read_lines_to_vec("data/sample.txt");
//
//     let mut result = 0;
//
//     // assert_eq!(result, );
//     answer(result);
// }

// fn parse_number<'b>(chars: &'static Vec<char>, mut pos: usize) -> (NodeType<'static>, usize) {
//     let mut n: Node<'b> = Node{a: NodeType::Null, b: NodeType::Null};
//     let c = chars[pos];
//     pos += 1;
//     match c {
//         '[' => {
//             let tmp = parse_number(chars, pos);
//             n.a = tmp.0;
//             pos = tmp.1 + 1;
//             let tmp = parse_number(chars, pos);
//             n.b = tmp.0;
//             pos = tmp.1 + 1;
//             return (NodeType::Node(&n), pos);
//         },
//         _ => {
//             return (NodeType::Number(c.to_digit(10).unwrap()), pos)
//         }
//     }
//
// }

// enum NodeType<'a> {
//     Number(u32),
//     Node(&'a Node<'a>),
//     Null,
// }
//
// struct Node<'a> {
//     a: NodeType<'a>,
//     b: NodeType<'a>,
// }
//
// impl fmt::Display for Node<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[")?;
//         if let NodeType::Number(a) = self.a {
//             write!(f, "{}", a)?;
//         } else if let NodeType::Node(a) = self.a {
//             write!(f, "{}", a)?;
//         }
//         write!(f, ",")?;
//         if let NodeType::Number(b) = self.b {
//             write!(f, "{}", b)?;
//         } else if let NodeType::Node(b) = self.b {
//             write!(f, "{}", b)?;
//         }
//         write!(f, "]")?;
//         Ok(())
//     }
// }

// impl Node {
//     fn add(self: &'static Self, rhs: &'static Self) -> Self {
//         Node{a: None, b: None, n: Some(self), m: Some(&rhs)}
//     }
// }

#[derive(Copy, Clone)]
struct Pair<T, U> {
    a: T,
    b: U
}

impl<T1, T2, U1, U2> ops::Add<Pair<U1, U2>> for Pair<T1, T2> {
    type Output = Pair<Pair<T1, T2>, Pair<U1, U2>>;
    fn add(self, rhs: Pair<U1, U2>) -> Self::Output {
        Pair{a: self, b: rhs}
    }
}

impl<T, U> fmt::Display for Pair<T, U>
where T: fmt::Display, U: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.a, self.b)
    }
}

