// Day 17
// Tetris

use std::cmp::max;
use std::collections::VecDeque;
use std::io::{self, BufRead};

mod cycle;

struct Cave {
    map: VecDeque<Vec<char>>,
    width: usize,
    height: usize,
    top: usize,       // the true height
    top_index: usize, // the height as indexed in the VecDeque map
    shape_idx: usize,
    wind_transform: Vec<char>,
    wind_idx: usize,
    floor: usize, // the offset of the floor to the true floor
}
impl Cave {
    fn new() -> Self {
        let width = 7;
        let height = 500;
        let floor = 0;

        let mut map = VecDeque::new();
        for _ in 0..height {
            map.push_back(vec!['.'; width]);
        }
        for x in 0..width {
            map[0][x] = '#';
        }
        let wind_transform = Vec::new();
        Cave {
            map,
            width,
            height,
            top: 0,
            top_index: 0,
            shape_idx: 0,
            wind_transform,
            wind_idx: 0,
            floor,
        }
    }

    fn is_obstructed(&self, shape_config: &[Vec<bool>], x: usize, y: usize) -> bool {
        let mut obstructed = false;
        let mut cy = y;
        'scan: for row in shape_config.iter() {
            let mut cx = x;
            for &cell in row {
                if self.map[cy][cx] != '.' && cell {
                    obstructed = true;
                    break 'scan;
                }
                cx += 1;
            }
            cy -= 1;
        }
        obstructed
    }

    fn load_shape(shape_idx: usize) -> Option<(Vec<Vec<bool>>, char)> {
        // tried to make these static but can't use vec!
        // it's so blazing fast i can't be bothered to investigate
        let HLINE: Vec<Vec<bool>> = vec![vec![true, true, true, true]];
        let PLUS: Vec<Vec<bool>> = vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ];
        let EL: Vec<Vec<bool>> = vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ];
        let VLINE: Vec<Vec<bool>> = vec![vec![true], vec![true], vec![true], vec![true]];
        let SQUARE: Vec<Vec<bool>> = vec![vec![true, true], vec![true, true]];

        if shape_idx == 0 {
            Some((HLINE, '#'))
        } else if shape_idx == 1 {
            Some((PLUS, '+'))
        } else if shape_idx == 2 {
            Some((EL, 'L'))
        } else if shape_idx == 3 {
            Some((VLINE, 'I'))
        } else if shape_idx == 4 {
            Some((SQUARE, 'O'))
        } else {
            None
        }
    }

    fn drop_rock(&mut self) {
        // get shape and fiddle index to load correct next shape
        let (shape_config, shape_char) = Cave::load_shape(self.shape_idx).unwrap();
        self.shape_idx = (self.shape_idx + 1) % 5;
        let shape_h = shape_config.len();
        let shape_w = shape_config[0].len();

        // xy addresses top corner of shape so add the shape height to y
        let mut x = 2; // always start two tiles from the left edge
        let mut y = self.top + 3 + shape_h; // and three tiles above the height

        // decrease y by floor to allow it to be used for indexing into map
        // we'll add it back again later to determine the final height
        y -= self.floor;

        // here we go
        'fall: loop {
            let t_char = self.wind_transform[self.wind_idx];
            self.wind_idx = (self.wind_idx + 1) % (self.wind_transform.len());

            // apply gas
            // only set transform if the shape will not be pushed oob
            let mut t: isize = 0;
            if t_char == '>' && (x + shape_w) < self.width {
                t = 1;
            } else if t_char == '<' && x > 0 {
                t = -1;
            }
            let dx: usize = (x as isize + t) as usize;

            // is the shape interacting with something
            // scan the shape for interaction
            let mut obstructed: bool;
            if dx != x {
                obstructed = self.is_obstructed(&shape_config, dx, y);
                if !obstructed {
                    x = dx;
                };
            }

            // let the rock fall
            let dy: usize = y - 1;
            obstructed = self.is_obstructed(&shape_config, x, dy);
            if !obstructed {
                y = dy;
                continue 'fall;
            };

            // if we are here we did not continue a fall transform
            // so we must be done falling
            let mut cy = y;
            for row in shape_config.iter() {
                let mut cx = x;
                for &cell in row {
                    if cell {
                        self.map[cy][cx] = shape_char;
                    }
                    cx += 1;
                }
                cy -= 1;
            }

            // update top if height has increased
            let new_top = cy + shape_h + self.floor;
            if new_top > self.top {
                self.top = max(self.top, new_top);
            }

            // if we're getting close to the top of the buffer
            // push some fresh rows to it
            if self.top_index > (self.height / 10) * 9 {
                for _ in 0..self.height / 10 {
                    self.map.pop_front();
                    self.map.push_back(vec!['.'; self.width]);
                    self.floor += 1; // raise the floor
                }
                for x in 0..self.width {
                    self.map[0][x] = '!';
                }
            }

            // update top index to account for changed top or floor
            self.top_index = self.top - self.floor;
            break 'fall;
        }
    }

    fn draw(&self) {
        for (row_i, row) in self.map.iter().rev().enumerate() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            print!(
                " < {} ({})",
                self.height - row_i - 1,
                self.height - row_i - 1 + self.floor
            );
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let mut cave = Cave::new();
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            cave.wind_transform.push(c);
        }
    }

    let mut rocks_dropped = 0;
    let l: usize = 1_000_000_000_000; // big boi
    let mut deltas: Vec<usize> = Vec::new();
    let mut last_cycle: Option<cycle::Cycle> = None;
    let big_height;
    loop {
        // part 1
        if rocks_dropped == 2022 {
            cave.draw();
            println!("Height at 2002 cycles: {}", cave.top);
        }

        let this_height = cave.top;
        cave.drop_rock();
        rocks_dropped += 1;
        deltas.push(cave.top - this_height); // keep track of height change

        // part 2
        // let a bunch of rocks fall and interrupt to check if
        // we've found a cycle in the height deltas to finish
        // the simulation
        if rocks_dropped % 1_000 == 0 {
            let this_cycle = cycle::detect_cycle(&deltas, 25); // find a cycle in height changes
            if last_cycle.is_some() && this_cycle.length > 0 && this_cycle == last_cycle.unwrap() {
                let initial_height: usize = deltas[0..this_cycle.start].iter().sum(); // tower height before cycling
                let height_per_cycle: usize = this_cycle.sequence.iter().sum();
                let whole_cycles = (l - this_cycle.start) / this_cycle.length;
                let remainder_cycle = (l - this_cycle.start) % this_cycle.length;
                let leftover_height: usize = this_cycle.sequence[0..remainder_cycle].iter().sum();
                big_height = initial_height + (height_per_cycle * whole_cycles) + leftover_height;
                break; // lets get out of here
            }
            last_cycle = Some(this_cycle);
        }
    }
    println!("Height at {} cycles: {}", l, big_height);
    println!("***");
}
