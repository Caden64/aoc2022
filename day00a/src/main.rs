// from https://github.com/timvisee/advent-of-code-2021/blob/master/day01a/src/main.rs

#![feature(array_windows)]
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Unable to find file");
    let str_depths = contents
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .array_windows()
        .filter(|[a,b]| a < b)
        .count();
    println!("{}", str_depths)
}

