// Day 1
// Calories
// Count top in list
// Count top three in list

// https://doc.rust-lang.org/std/keyword.self.html
// this is shorthand for
// use std::io;
// use std::io::BufRead;
use std::io::{self, BufRead};

fn main() {
    let mut index = 1;
    let mut current = 0;
    let mut max_v = 0;
    let mut max_i = -1;

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    let mut vec: Vec<u32> = Vec::new();

    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        if this_line.is_empty() { // rather than ""
            if current > max_v {
                max_i = index;
                max_v = current;
            }
            vec.push(current);
            println!("Elf {} carries {} calories. Next!", index, current);
            current = 0;
            index += 1;
        } else {
            println!("{}", this_line);
            current += this_line.parse::<u32>().unwrap();
        }
    }
    // last
    if current > max_v {
        max_i = index;
        max_v = current;
    }
    vec.push(current);
    println!("Elf {} carries {} calories. Next!", index, current);

    // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html#sort-a-vector-of-integers
    vec.sort();
    let tail = vec.split_off(vec.len() - 3); // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off
    let sum: u32 = tail.iter().sum();

    println!("***");
    println!("Best elf {} carries {} calories!", max_i, max_v);
    println!("Top 3 elves carry {:?} calories, totalling {}!", tail, sum);
}
