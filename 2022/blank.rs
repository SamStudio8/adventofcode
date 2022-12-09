// Day N
// Hoots

// https://doc.rust-lang.org/std/keyword.self.html
// this is shorthand for
// use std::io;
// use std::io::BufRead;
use std::io::{self, BufRead};

fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        println!("{}", this_line);
    }
    println!("***");
}
