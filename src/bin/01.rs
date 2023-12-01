use std::fs::File;
use std::io::{BufRead, BufReader};

const DIGITS: &'static [&str] = &["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn read_lines() -> impl Iterator<Item = String> {
    let file = File::open("inputs/01.txt").expect("input file not present");
    BufReader::new(file).lines().map(|l| l.expect("error reading from file"))
}

fn digits<'a>(s: &'a str) -> impl Iterator<Item = usize> + 'a {
    s.chars().filter_map(|c| c.to_digit(10)).map(|d| d as usize)
}

fn map_to_digit(s: &str) -> Option<usize> {
    if let Some(Some(d)) = s.chars().next().map(|c| c.to_digit(10)) { Some(d as usize) }
    else { (1 ..= 9).filter(|&d| s.starts_with(DIGITS[d])).next() }
}

fn true_digits<'a>(s: &'a str) -> impl Iterator<Item = usize> + 'a {
    (0 .. s.len()).filter_map(|idx| map_to_digit(&s[idx..]))
}

fn calibration_value<I>(digits: I) -> usize where I: Iterator<Item = usize> {
    let mut peekable = digits.peekable();
    let first = *peekable.peek().expect("no first element");
    let last = peekable.last().expect("no last element");
    10 * first + last
}

fn main() {
    let lines: Vec<_> = read_lines().collect();
    println!("Part 1: {}", lines.iter().map(|l| calibration_value(digits(l))).sum::<usize>());
    println!("Part 2: {}", lines.iter().map(|l| calibration_value(true_digits(l))).sum::<usize>());
}
