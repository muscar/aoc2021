use std::collections::HashMap;

use crate::input::read_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

fn bresenham(s: &Segment) -> Vec<Point> {
    let p1 = s.p1;
    let p2 = s.p2;
    let mut ps = Vec::new();
    let dx = (p2.x - p1.x).abs();
    let dy = -(p2.y - p1.y).abs();
    let sx = (p2.x - p1.x).signum();
    let sy = (p2.y - p1.y).signum();
    let mut err = dx + dy;
    let mut x0 = p1.x;
    let mut y0 = p1.y;
    loop {
        ps.push(Point { x: x0, y: y0 });
        if x0 == p2.x && y0 == p2.y {
            break;
        }
        let e2 = err * 2;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    ps
}

fn part1(ps: &[Segment]) -> i64 {
    ps.iter()
        .filter(|s| s.p1.x == s.p2.x || s.p1.y == s.p2.y)
        .flat_map(bresenham)
        .fold(HashMap::new(), |mut acc, p| {
            *acc.entry(p).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, c)| **c > 1)
        .count() as i64
}

fn part2(ps: &[Segment]) -> i64 {
    ps.iter()
        .flat_map(bresenham)
        .fold(HashMap::new(), |mut acc, p| {
            *acc.entry(p).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, c)| **c > 1)
        .count() as i64
}

fn parse_segment(l: String) -> Segment {
    let mut ps = l.split(" -> ").map(|p| {
        let mut ps = p.split(",").map(|s| s.parse::<i64>().unwrap());
        Point {
            x: ps.next().unwrap(),
            y: ps.next().unwrap(),
        }
    });
    Segment {
        p1: ps.next().unwrap(),
        p2: ps.next().unwrap(),
    }
}

pub fn run() -> ((i64, i64), (i64, i64)) {
    let input_small = read_input("input.day5.small", parse_segment);
    let input_large = read_input("input.day5.large", parse_segment);
    (
        (part1(&input_small), part2(&input_small)),
        (part1(&input_large), part2(&input_large)),
    )
}
