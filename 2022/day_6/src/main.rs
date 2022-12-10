// Day 6
// Comms

// https://doc.rust-lang.org/std/keyword.self.html
// this is shorthand for
// use std::io;
// use std::io::BufRead;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::env;


fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing message size!");
    }
    let comms_len: usize = args[1].parse().unwrap();

    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();

        let mut pos = 0;
        while pos <= (this_line.chars().count() - comms_len) {
            let slice = &this_line[pos..pos+comms_len];
            let slice_set: HashSet<char> = slice.chars().collect();
            if slice_set.len() == comms_len {
                println!("{}@{}", slice, pos + comms_len);
                break;
            }
            pos += 1;
        }
    }
}
