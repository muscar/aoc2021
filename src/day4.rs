use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn mark(n: i64, board: &mut Vec<Vec<Option<i64>>>) /*-> Vec<Vec<Option<i64>>>*/
{
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if matches!(board[i][j], Some(x) if x == n) {
                board[i][j] = None;
                break;
            }
        }
    }
}

fn wins(b: &Vec<Vec<Option<i64>>>) -> bool {
    let has_row = b.iter().any(|r| r.iter().all(|n| n.is_none()));
    let has_col = (0..b.len())
        .map(|i| b.iter().map(|r| r[i]).collect::<Vec<_>>())
        .any(|r| r.iter().all(|n| n.is_none()));
    has_row || has_col
}

fn part1(ns: &[i64], mut boards: Vec<Vec<Vec<Option<i64>>>>) -> i64 {
    let mut winner_n = 0;
    let mut winner_idx = 0;
    'play: for n in ns {
        for i in 0..boards.len() {
            mark(*n, &mut boards[i]);
            if wins(&boards[i]) {
                winner_n = *n;
                winner_idx = i;
                break 'play;
            }
        }
    }
    boards[winner_idx]
        .iter()
        .flat_map(|r| r.iter().filter_map(|n| *n))
        .sum::<i64>()
        * winner_n
}

fn part2(ns: &[i64], mut boards: Vec<Vec<Vec<Option<i64>>>>) -> i64 {
    let mut winner_n = 0;
    let mut winner_idx = 0;
    let mut winners = HashSet::new();
    'l: for n in ns {
        for i in 0..boards.len() {
            if winners.contains(&i) {
                continue;
            }
            mark(*n, &mut boards[i]);
            if wins(&boards[i]) {
                winner_n = *n;
                winner_idx = i;
                winners.insert(i);
                if winners.len() == boards.len() {
                    break 'l;
                }
            }
        }
    }
    boards[winner_idx]
        .iter()
        .flat_map(|r| r.iter().filter_map(|n| *n))
        .sum::<i64>()
        * winner_n
}

fn read_input(path: &str) -> (Vec<i64>, Vec<Vec<Vec<Option<i64>>>>) {
    let reader = BufReader::new(File::open(path).expect("failed to open input file"));
    let mut lines = reader.lines();
    let ns = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    lines.next();

    let mut boards = Vec::new();
    let mut curr_board = Vec::new();
    while let Some(s) = lines.next() {
        if s.as_ref().unwrap().len() == 0 {
            boards.push(curr_board.clone());
            curr_board.clear();
            continue;
        }
        let row = s
            .unwrap()
            .split(' ')
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>();
        curr_board.push(row);
    }
    boards.push(curr_board.clone());
    (ns, boards)
}

pub fn run() -> ((i64, i64), (i64, i64)) {
    let (ns_small, boards_small) = read_input("input.day4.small");
    let (ns_large, boards_large) = read_input("input.day4.large");
    (
        (
            part1(&ns_small, boards_small.clone()),
            part2(&ns_small, boards_small),
        ),
        (
            part1(&ns_large, boards_large.clone()),
            part2(&ns_large, boards_large),
        ),
    )
}
