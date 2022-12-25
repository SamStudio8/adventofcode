// Day 25
// Fuel

use std::io::{self, BufRead};

fn snafu_symbol_to_num(c: char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("bad snafu!"),
    }
}
fn snafu_to_dec(s: &str) -> isize {
    let mut num = 0;
    for (i, c) in s.chars().rev().enumerate() {
       let char_val = snafu_symbol_to_num(c);
       num += 5_isize.pow(i as u32) * char_val;
    }
    num
}

// its too early for this
fn dec_to_snafu(mut num: isize) -> String {
    let mut snafu = String::new();
    while num > 0 {
        let rem = num % 5; // use this for snafu char
        num /= 5;
        if rem <= 2 {
            snafu.push(char::from_digit(rem as u32, 10).unwrap());
        }
        else if rem == 3 {
            snafu.push('='); // f
            num += 1; // roll over to next pos
        }
        else if rem == 4 {
            snafu.push('-');
            num += 1; // roll over to next pos
        }
    }
    snafu.chars().rev().collect::<String>()
}

fn main() {

    let mut sum = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        sum += snafu_to_dec(&this_line);
    }
    println!("Balloon Fuel: {}", sum);
    let snafu = dec_to_snafu(sum);
    println!("Balloon SNAFU: {}", snafu);
}
