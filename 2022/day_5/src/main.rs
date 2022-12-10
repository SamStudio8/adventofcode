// Day 5
// Crates and cranes

// https://doc.rust-lang.org/std/keyword.self.html
// this is shorthand for
// use std::io;
// use std::io::BufRead;
use std::io::{self, BufRead};
use regex::Regex;

// https://stackoverflow.com/a/58119924
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    //let collapse_space = Regex::new(r"(\s+)").unwrap();
    let capture_spec = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut crate_spec: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        if this_line.is_empty() {
            break;
        }
        crate_spec.push(this_line.clone());
    }
    crate_spec.reverse();
    println!("***");

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for (i, line) in crate_spec.iter().enumerate() {
        if i == 0 {
            // Get last element after split for number of stacks
            let n_stacks: i8 = line.trim().split(' ').last().expect("oh no").parse().unwrap();
            for _ in 0..n_stacks {
                stacks.push(Vec::new());
            }
            continue;
        }

        // Cobble together a config
        let mut pos = 1;
        let mut res: Vec<char> = Vec::new();
        while pos < line.len() {
            let c: char = line.chars().nth(pos).unwrap(); // i think this is bad perf
            res.push(c);
            pos += 4; // lol
        }

        println!("{:?}", res);
        for (stack_i, &stack_char) in res.iter().enumerate() {
            if stack_char != ' ' {
                stacks[stack_i].push(stack_char);
            }
        }
    }

    // hello stacks
    for (stack_i, stack) in stacks.iter().enumerate() {
        println!("{}>{:?}", stack_i, stack);
    }

    let mode = "9001";

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();

        let n_to_move = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let src = matches.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let dest = matches.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        println!("MV {} {}>{}", n_to_move, src, dest);

        // hello stacks
        for (stack_i, stack) in stacks.iter().enumerate() {
            println!("{}>{:?}", stack_i, stack);
        }

        if mode == "9001" {
            let src_len = stacks[src].len();
            let crane_stack = stacks[src].split_off(src_len - n_to_move as usize);
            stacks[dest].extend_from_slice(&crane_stack);
        }
        else {
            for _ in 0..n_to_move {
                let this_crate = stacks[src].pop().unwrap();
                stacks[dest].push(this_crate);
            }
        }

    }

    // answer the question
    let mut the_answer: Vec<char> = Vec::new();
    for (stack_i, stack) in stacks.iter_mut().enumerate() {
        // assigning from a block woaoaaoaoah!
        let this_crate: char = {
            if stack.is_empty() {
                ' '
            }
            else {
                stack.pop().unwrap()
            }
        };
        the_answer.push(this_crate);
        println!("{}>{:?}", stack_i, this_crate);
    }

    println!("***");
    let the_answer_str: String = the_answer.iter().collect();
    println!("Using crane model {}!", mode);
    println!("{}", the_answer_str);

}
