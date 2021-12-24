use crate::utils::{day, close, part, read_lines_to_vec, answer};
use std::collections::HashSet;

pub fn run() {
    day(17);
    part1();
    part2();
    close();
}

fn part1() {
    part(1);

    let input = read_lines_to_vec("data/day17.txt");
    let mut input = input[0].strip_prefix("target area: ").unwrap().split(", ");
    let x: Vec<i32> = input.next().unwrap().strip_prefix("x=").unwrap().split("..").map(|x| x.parse().unwrap()).collect();
    let x0 = x[0];
    let x1 = x[1];
    let y: Vec<i32> = input.next().unwrap().strip_prefix("y=").unwrap().split("..").map(|y| y.parse().unwrap()).collect();
    let y0 = y[0];
    let y1 = y[1];

    let mut ymax = i32::MIN;

    for y in y0..=y1 {
        let vmax = y.abs();
        for vy in 1..=vmax {
            let n0 = 0.5 * (2.0 * vy as f64 + 1.0);
            let ytmp = (n0 * vy as f64 - 0.5 * n0 * (n0 - 1.0)) as i32;

            let n = num_steps(y, vy);
            for x in x0..=x1 {
                if let Some(n1) = n.0 {
                    if let Some(vx) = x_vel(x, n1) {
                        let pos = pos_n(n1, vx, vy);
                        assert_eq!(pos.0, x);
                        assert_eq!(pos.1, y);
                        if ytmp > ymax {
                            ymax = ytmp;
                        }
                    }
                }
                if let Some(n2) = n.1 {
                    if let Some(vx) = x_vel(x, n2) {
                        let pos = pos_n(n2, vx, vy);
                        assert_eq!(pos.0, x);
                        assert_eq!(pos.1, y);
                        if ytmp > ymax {
                            ymax = ytmp;
                        }
                    }
                }
            }
        }
    }

    let result = ymax;

    assert_eq!(result, 9870);
    answer(result);
}

fn part2() {
    part(2);

    let input = read_lines_to_vec("data/day17.txt");
    let mut input = input[0].strip_prefix("target area: ").unwrap().split(", ");
    let x: Vec<i32> = input.next().unwrap().strip_prefix("x=").unwrap().split("..").map(|x| x.parse().unwrap()).collect();
    let x0 = x[0];
    let x1 = x[1];
    let y: Vec<i32> = input.next().unwrap().strip_prefix("y=").unwrap().split("..").map(|y| y.parse().unwrap()).collect();
    let y0 = y[0];
    let y1 = y[1];

    let mut set = HashSet::new();

    for y in y0..=y1 {
        let vmax = y.abs();
        let vmin = -vmax;
        for vy in vmin..=vmax {
            let n = num_steps(y, vy);
            for x in x0..=x1 {
                if let Some(n1) = n.0 {
                    if let Some(vx) = x_vel(x, n1) {
                        let pos = pos_n(n1, vx, vy);
                        assert_eq!(pos.0, x);
                        assert_eq!(pos.1, y);
                        set.insert((vx, vy));
                    }
                }
                if let Some(n2) = n.1 {
                    if let Some(vx) = x_vel(x, n2) {
                        let pos = pos_n(n2, vx, vy);
                        assert_eq!(pos.0, x);
                        assert_eq!(pos.1, y);
                        set.insert((vx, vy));
                    }
                }
            }
        }
    }

    let result = set.len();

    assert_eq!(result, 5523);
    answer(result);
}

fn pos_n(n: i32, vx: i32, vy: i32) -> (i32, i32) {
    let nx = if n > vx.abs() { vx.abs() } else { n };
    let s = if vx < 0 { -1 } else { 1 };
    (nx * vx - s * nx * (nx-1) / 2, n * vy - s * n * (n-1) / 2)
}

fn num_steps(p: i32, v: i32) -> (Option<i32>, Option<i32>) {
    let a = 2.0*p as f64;
    let b = 2.0*v as f64 + 1.0;
    let c = f64::sqrt(0.25*b*b - a);
    let n1 = 0.5*b + c;
    let n2 = 0.5*b - c;

    let r1 = (n1 - (n1 as i32) as f64).abs();
    let n1 = if r1 < 1e-7 && n1 >= 0.0 {
        Some(n1 as i32)
    } else {
        None
    };

    let r2 = (n2 - (n2 as i32) as f64).abs();
    let n2 = if r2 < 1e-7 && n2 >= 0.0 {
        Some(n2 as i32)
    } else {
        None
    };

    (n1, n2)
}

fn x_vel(x: i32, n: i32) -> Option<i32> {
    let x = x as f64;
    let n = n as f64;
    let vx = x / n + 0.5 * (n - 1.0);
    if n > vx {
        let nu = f64::sqrt(2.0 * x as f64);
        let n1 = nu.floor();
        let n2 = nu.ceil();

        let v1 = x / n1 + 0.5 * (n1 - 1.0);
        let v2 = x / n2 + 0.5 * (n2 - 1.0);

        if v1 < v2 {
            let r = (v1 - (v1 as i32) as f64).abs();
            if r < 1e-7 {
                Some(v1 as i32)
            } else {
                None
            }
        } else {
            let r = (v2 - (v2 as i32) as f64).abs();
            if r < 1e-7 {
                Some(v2 as i32)
            } else {
                None
            }
        }
    } else {
        let r = (vx - (vx as i32) as f64).abs();
        if r < 1e-7 {
            Some(vx as i32)
        } else {
            None
        }
    }
}
