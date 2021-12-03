use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_input<F, T>(path: &str, f: F) -> Vec<T>
where
    F: FnMut(String) -> T,
{
    let reader = BufReader::new(File::open(path).expect("failed to open input file"));
    reader
        .lines()
        .map(|s| s.expect("failed to read line"))
        .map(f)
        .collect::<Vec<_>>()
}

pub fn split_n<'a, T: Default + Copy + From<&'a str>, const N: usize>(
    s: &'a str,
    pat: char,
) -> [T; N] {
    let mut result = [Default::default(); N];
    let mut it = s.split(pat).take(N).enumerate();
    while let Some((idx, x)) = it.next() {
        result[idx] = T::try_from(x).unwrap();
    }
    result
}
