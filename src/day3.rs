use crate::input::read_input;

fn part1(ns: &[String]) -> i64 {
    let mut e = 1 << ns[0].len() - 1;
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..ns[0].len() {
        let c = ns.iter().filter(|s| s.as_bytes()[i] == b'0').count();
        if c > ns.len() / 2 {
            epsilon |= e;
        } else {
            gamma |= e;
        }
        e >>= 1;
    }
    gamma * epsilon
}

fn find_rating(mut ns: Vec<&String>, criteria: u8) -> String {
    let mut k = 0;
    while ns.len() > 1 {
        let c = ns.iter().filter(|s| s.as_bytes()[k] == b'0').count();
        let mcb = if c > ns.len() / 2 { 0 } else { 1 };
        ns.retain(|s| s.as_bytes()[k] - b'0' & 0x01 == mcb ^ criteria);
        k += 1;
    }
    ns[0].to_owned()
}

fn part2(ns: &[String]) -> i64 {
    let oxygen_rating = find_rating(ns.iter().collect::<Vec<_>>(), 0);
    let co2_rating = find_rating(ns.iter().collect::<Vec<_>>(), 1);
    i64::from_str_radix(&oxygen_rating, 2).unwrap() * i64::from_str_radix(&co2_rating, 2).unwrap()
}

pub fn run() -> ((i64, i64), (i64, i64)) {
    let input_small = read_input("input.day3.small", |l| l);
    let input_large = read_input("input.day3.large", |l| l);
    (
        (part1(&input_small), part2(&input_small)),
        (part1(&input_large), part2(&input_large)),
    )
}
