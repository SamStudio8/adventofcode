// Day N
// Hoots

use std::io::{self, BufRead};
use regex::Regex;

fn main() {

    let capture_spec = Regex::new(r"Hoot at x=(?P<x>-?\d+), y=(?P<y>-?\d+)").unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();
        println!("{}", this_line);
    }
    println!("***");
}
