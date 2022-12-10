// Day 4
// Overlaps

// https://doc.rust-lang.org/std/keyword.self.html
// this is shorthand for
// use std::io;
// use std::io::BufRead;
use std::io::{self, BufRead};

// cargo add regex
// https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
use regex::Regex;

use std::cmp::{min, max};


fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    let re: Regex = Regex::new(
        r"(\d+)-(\d+),(\d+)-(\d+)"
    ).unwrap();

    let mut duplicates = 0;
    let mut duplicates_2 = 0;
    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        println!("{}", this_line);
        let matches = re.captures(&this_line).unwrap();

        let start_1 = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let start_2 = matches.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let end_1 = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let end_2 = matches.get(4).unwrap().as_str().parse::<i32>().unwrap();

        // if the max start is before the min end
        // there must be a point of overlap
        let olen = min(end_1, end_2) - max(start_1, start_2);
        println!("{}", olen);
        if olen >= 0 {
            duplicates_2 += 1;
            if olen == min(end_1-start_1, end_2-start_2) {
                // if there is overlap and the overlap is the length of the smaller interval
                duplicates += 1;
            }
        }

    }
    println!("***");
    println!("Dupes: {}", duplicates);
    println!("Dupes: {}", duplicates_2);
}
