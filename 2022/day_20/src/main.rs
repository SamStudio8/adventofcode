// Day 20
// Mixing

use std::io::{self, BufRead};

fn main() {

    let mix_setting = 10;
    let decryption_key = 811589153;
    let coordinate_keys = [1000, 2000, 3000];

    let mut number_vec = Vec::new();
    let mut mixer = Vec::new();

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        let this_val = line.unwrap().parse::<isize>().unwrap() * decryption_key;
        number_vec.push(this_val);
        mixer.push( (i, this_val) );
    }
    let n: isize = number_vec.len() as isize;

    for _ in 0..mix_setting {
        for (number_i, &number) in number_vec.iter().enumerate() {
            // don't bother popping and inserting zeroes
            if number != 0 {
                // get where the pair is now
                let current_index = mixer.iter().position(|&(idx, _)| idx == number_i).unwrap();
                mixer.remove(current_index); // pop it out

                // as lovely as this is we don't need it now thanks to rem_euclid
                // map negative number to start of list, apply mod as if it
                // were positive and then take the rem off the list end
                // if number < 0 {
                //     let new_number = number.abs() - current_index as isize;
                //     new_index = (n-1) - (new_number % (n-1));
                // }
                let new_index = (current_index as isize + number).rem_euclid(n - 1);
                if new_index == 0 {
                    // gotcha! new_index of 0 means the index divides with no rem
                    // so it goes at the end of the list and not the start
                    mixer.push((number_i, number));
                }
                else {
                    mixer.insert(new_index as usize, (number_i, number));
                }
            }
        }
    }
    let zero_index = mixer.iter().position(|&(_, val)| val == 0).unwrap();
    let mut coord_sum = 0;
    for k in coordinate_keys {
        coord_sum += mixer[(zero_index + k) % n as usize].1;
    }
    println!("Grove co-ordinate sum: {}", coord_sum);
}
