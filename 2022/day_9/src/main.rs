// Day 9
// Knots

use std::io::{self, BufRead};
use std::collections::HashSet;
use std::cmp::Ordering;


// writing `&Vec` instead of `&[_]` involves a new object where a slice will do
// change &Vec<(isize, isize)>, to: `&[(isize, isize)]`
fn array_from_knots(knots: &[(isize, isize)], (min_x, min_y, width, height): (isize, isize, isize, isize)) -> Vec<Vec<char>> {
    let mut array = vec![vec!['.'; width as usize]; height as usize];
    for (v_i, v) in knots.iter().enumerate() {
        let _x: isize = v.0 + (min_x.abs());
        let _y: isize = v.1 + (min_y.abs());
        array[_y as usize][_x as usize] = char::from_digit(v_i as u32, 10).unwrap();
    }
    array
}

fn draw_array(array: Vec<Vec<char>>) {
    for row in array.iter().rev() {
        for res in row.iter() {
            print!("{}", res);
        }
        println!();
    }
}

fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    let mut positions_t: HashSet<(isize, isize)> = HashSet::new();
    positions_t.insert((0,0));

    let draw = false;
    let draw_all = false;
    let num_knots = 9; // set to 2 for part 1
    let mut knots: Vec<(isize, isize)> = Vec::new();
    for _ in 0..num_knots {
        knots.push( (0, 0) );
    }

    // just set these manually so we can plot as we go along
    let min_x: isize = -20;
    let min_y: isize = -20;
    let max_x = 20;
    let max_y = 20;

    let width = 1 + (max_x - min_x);
    let height = 1 + (max_y - min_y);

    let mut is_diagonal: bool;

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();

        let fields = this_line.split_once(' ').unwrap();
        println!("{:?}", fields);

        for iter_i in 0..fields.1.parse().unwrap() {
            // move head
            let (hx, hy) = &mut knots[0]; // fancy tuple deref!
            match fields.0 {
                "R" => *hx += 1,
                "U" => *hy += 1,
                "L" => *hx -= 1,
                "D" => *hy -= 1,
                _ => panic!("Invalid move {}!", fields.0)
            }

            // move rest of knots iteratively
            for knot_i in 1..num_knots {
                let (hx, hy) = knots[knot_i - 1];
                let (mut tx, mut ty) = knots[knot_i];

                is_diagonal = false;
                if hx != tx && hy != ty {
                    is_diagonal = true;
                }
                println!("ITER {} - KNOT {}: HEAD@{}-{}, THIS@{}-{} Diag:{}", iter_i+1, knot_i, hx, hy, tx, ty, is_diagonal);

                // skip if no update needed
                if (hx-tx).abs() <= 1 && (hy-ty).abs() <= 1 {
                    println!("Skipping knot.{}", knot_i);
                    continue;
                }

                // cargo clippy suggested these would be more rusty as match
                // let mut dx = 0;
                // if hx > tx {
                //     dx = 1;
                // }
                // else if hx < tx {
                //     dx = -1;
                // }
                let dx = match hx.cmp(&tx) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                };

                let dy = match hy.cmp(&ty) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                };

                if (hx - tx).abs() == 2 {
                    tx += dx;
                    if is_diagonal {
                        ty += dy;
                    }
                }
                else if (hy - ty).abs() == 2 {
                    ty += dy;
                    if is_diagonal {
                        tx += dx;
                    }
                }

                // update
                knots[knot_i] = (tx, ty);
                println!("Moved knot.{} to ({}, {})", knot_i, tx, ty);
                println!("ITER {} - KNOT {}", iter_i+1, knot_i);
                if draw_all {
                    let array = array_from_knots(&knots, (min_x, min_y, width, height));
                    draw_array(array);
                }

                // keep track of where the tailiest tail has been to
                // actually answer the question
                if knot_i == (num_knots-1) {
                    positions_t.insert( (tx, ty) );
                }
            }
        }
        if draw {
            let array = array_from_knots(&knots, (min_x, min_y, width, height));
            draw_array(array);
        }
    }
    println!("***");
    println!("Unique positions: {}", positions_t.len());

    let mut array = vec![vec!['.'; width as usize]; height as usize];
    for v in positions_t.iter() {
        let _x: isize = v.0 + (min_x.abs());
        let _y: isize = v.1 + (min_y.abs());
        array[_y as usize][_x as usize] = '#';
    }
    draw_array(array);


}
