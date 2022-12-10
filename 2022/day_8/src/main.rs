// Day 8
// Tree visibility

use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();

    // read lines to make grid
    let mut tree_spec: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        tree_spec.push(line.unwrap().clone());
    }

    // determine grid size
    // ruin nice code with isize nonsense
    let mut iwidth: isize = 0;
    let iheight: isize = tree_spec.len() as isize;
    for (i, line) in tree_spec.iter().enumerate() {
        if i == 0 {
            iwidth = line.len() as isize;
        }
        println!("{}", line);
    }

    // construct grid
    let mut array = vec![vec![0; iwidth as usize]; iheight as usize];
    for (x, line) in tree_spec.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            array[x][y] = c.to_digit(10).unwrap() as usize;
        }
    }

    // does it look like a grid
    println!("{:?}", array);

    let mut total_visible = 0;
    let mut max_scene_score = 0;
    let transform: [(isize, isize);4] = [
        (-1,  0), // left
        ( 0,  1), // up
        ( 1,  0), // right
        ( 0, -1), // down
    ];

    // lets try the terrible way
    for (x, row) in array.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            let mut scene_score = 1; // init at 1 coz we multiply, idiot
            let mut visible = false;

            for tt in transform {
                let mut side_visible = true;
                let mut side_score = 0;

                let mut x_t = (x as isize) + tt.0;
                let mut y_t = (y as isize) + tt.1;
                // check we're still in the grid
                // (side score is 0 if we're not)
                while x_t >= 0 && x_t <= (iheight-1) && y_t >= 0 && y_t <= (iwidth-1) {
                    let val = array[x_t as usize][y_t as usize];
                    side_score += 1;
                    if val >= cell {
                        // view obstructed by tree of same or higher length
                        side_visible = false;
                        break;
                    }
                    // transform again
                    x_t += tt.0;
                    y_t += tt.1;
                }
                //println!("Side score @{}-{}: {:?}={}", x+1, y+1, tt, side_score);
                scene_score *= side_score;
                if side_visible {
                    visible = true;
                }
            }
            if visible {
                total_visible += 1;
            }
            if scene_score > max_scene_score {
                max_scene_score = scene_score;
            }
            //println!("Scene score @{}-{}={}", x+1, y+1, scene_score);
        }
    }
    println!("***");
    println!("Total visible: {}", total_visible);
    println!("Max scene score: {}", max_scene_score);

}
