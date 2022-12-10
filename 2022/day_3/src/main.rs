// Day 3
// Backpacks

// https://doc.rust-lang.org/std/keyword.self.html
// this is shorthand for
// use std::io;
// use std::io::BufRead;
use std::io::{self, BufRead};

use std::collections::HashSet;
//use std::collections::HashMap;

// https://stackoverflow.com/a/58119924
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_priority(common_char: char) -> u32 {
    // this line panics, i think to_digit is still unstable?
    // https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
    //let index: u32 = common_char.to_digit(10).unwrap();

    // force to number with as instead
    let mut priority: u32 = common_char as u32;
    if common_char.is_lowercase() {
        priority -= 96;
    }
    else {
        // -64 + 26 = -38
        priority -= 38;
    }
    // return priority;
    priority
}

fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    // needs to hold String (not &str) to keep values not refs
    let mut pack_lines: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];

    let mut total = 0;
    let mut total2 = 0;
    let mut line_number = 0;
    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        //println!("{}", this_line);

        let halfway = this_line.chars().count() / 2;
        let this_split = this_line.split_at(halfway);
        //println!("{:?}", this_split);

        let left: HashSet<char> = this_split.0.chars().collect();
        let right: HashSet<char> = this_split.1.chars().collect();

        // this is std::collections::hash::set::Intersection
        // need mut to call `next` to advance the iterator, using .last() instead
        let intersection = left.intersection(&right);
        //print_type_of(&intersection);

        let common_char: char = *intersection.last().unwrap();
        let priority: u32 = get_priority(common_char);
        total += priority;

        pack_lines[line_number] = this_line.clone();

        line_number += 1;
        if line_number == 3 {
            // woops duplicates are allowed hnhnhnhn
            //let mut freq: HashMap<char, i32> = HashMap::new();
            //for (i, p) in pack_lines.iter().enumerate() {
            //    println!("{}: {}", i, p);
            //    for c in p.chars() {
            //        *freq.entry(c).or_insert(0) += 1;
            //    }
            //}
            // get the max
            // https://stackoverflow.com/questions/62525693/how-do-i-get-the-key-associated-with-the-maximum-value-of-a-rust-hashmap
            //let common_char = freq
            //    .iter()
            //    .max_by(|a, b| a.1.cmp(&b.1))
            //    .map(|(k, _v)| k)
            //    .unwrap();

            let mut pack: HashSet<char> = pack_lines[0].chars().collect();
            for p in pack_lines.iter() {
                let this_pack: HashSet<char> = p.chars().collect();
                // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain
                pack.retain(|x| this_pack.contains(x));
            }
            println!("* ({:?})", pack);
            let common_char2 = *pack.iter().next().unwrap();

            line_number = 0;
            let priority: u32 = get_priority(common_char2);
            println!("* {:?} ({:?})", common_char, priority);
            total2 += priority;
        }

        println!("{:?} & {:?} = {:?} ({:?})", left, right, common_char, priority);

    }
    println!("***");
    println!("Total score: {}", total);
    println!("Total part 2 score: {}", total2);
}
