// Day 10
// Pixels

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut cycle = 0;
    let mut register: isize = 1;
    let mut addx: isize;
    let mut to_cycle;

    // init screen
    let width: isize = 40;
    let height: isize = 6;
    let mut screen = vec![vec!['.'; width as usize]; height as usize];

    let interrupts = [20, 60, 100, 140, 180, 220, usize::MAX];
    let mut next_interrupt_idx = 0;
    let mut next_interrupt = interrupts[next_interrupt_idx];

    let mut cycle_scores = Vec::new();

    let pixel = [-1, 0, 1];

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();

        // assume one cycle with nothing to add
        to_cycle = 1;
        addx = 0;

        if this_line.starts_with("addx") {
            let fields = this_line.split_once(' ').unwrap();
            to_cycle = 2;
            addx = fields.1.parse::<isize>().unwrap();
        }

        // always cycle once for noops and addx init
        // set to_cycle = 2 for addx
        for _ in 0..to_cycle {
            cycle += 1;
            if cycle == next_interrupt {
                // inspect the cycle
                let cycle_score = register * (cycle as isize);
                println!(
                    "Register {} Score {} @ Cycle {}",
                    register, cycle_score, cycle
                );
                next_interrupt_idx += 1;
                next_interrupt = interrupts[next_interrupt_idx];
                cycle_scores.push(cycle_score);
            }
            // draw pixel
            let pixel_y = (cycle - 1) / (width as usize);
            let pixel_x = (cycle - 1) % (width as usize);
            for transform in pixel {
                let sprite_loc = (register + transform) as usize;
                if sprite_loc == pixel_x {
                    screen[pixel_y][pixel_x] = '#';
                }
            }
        }
        register += addx;
    }
    println!("***");
    println!("Signal strength: {}", cycle_scores.iter().sum::<isize>());

    for row in screen.iter() {
        for res in row.iter() {
            print!("{}", res);
        }
        println!();
    }
}
