use crate::input::read_input;

fn part1(ns: &[i64]) -> i64 {
    ns.windows(2).filter(|w| w[1] > w[0]).count() as i64
}

fn part2(ns: &[i64]) -> i64 {
    let ws = ns.windows(3).map(|w| w.iter().sum::<i64>());
    ws.clone().skip(1).zip(ws).filter(|(m, n)| m > n).count() as i64
}

fn part2_1(ns: &[i64]) -> i64 {
    ns.windows(4).filter(|w| w[3] > w[0]).count() as i64
}

pub fn run() -> ((i64, i64), (i64, i64)) {
    let input_small = read_input("input.day1.small", |l| {
        l.parse().expect("failed to parse entry")
    });
    let input_large = read_input("input.day1.large", |l| {
        l.parse().expect("failed to parse entry")
    });
    (
        (part1(&input_small), part2_1(&input_small)),
        (part1(&input_large), part2_1(&input_large)),
    )
}
