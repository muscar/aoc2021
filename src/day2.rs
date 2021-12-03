use crate::input::{read_input, split_n};

fn part1(cmds: &[(String, i64)]) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    for (cmd, arg) in cmds {
        match cmd.as_str() {
            "forward" => pos += arg,
            "down" => depth += arg,
            "up" => depth -= arg,
            _ => panic!(),
        }
    }
    pos * depth
}

fn part2(cmds: &[(String, i64)]) -> i64 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (cmd, arg) in cmds {
        match cmd.as_str() {
            "forward" => {
                pos += arg;
                depth += aim * arg;
            }
            "down" => aim += arg,
            "up" => aim -= arg,
            _ => panic!(),
        }
    }
    pos * depth
}

pub fn run() -> ((i64, i64), (i64, i64)) {
    let parse_line = |l: String| {
        let parts = split_n::<&str, 2>(&l, ' ');
        (
            parts[0].to_owned(),
            parts[1].parse::<i64>().expect("failed to parse entry"),
        )
    };
    let input_small = read_input("input.day2.small", parse_line);
    let input_large = read_input("input.day2.large", parse_line);
    (
        (part1(&input_small), part2(&input_small)),
        (part1(&input_large), part2(&input_large)),
    )
}
