// Day 14
// Sand

use regex::Regex;
use std::cmp::{max, min};
use std::io::{self, BufRead};

struct Cave {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    x_bounds: (usize, usize),
    y_bounds: (usize, usize),
    sand_pos: (usize, usize),
}
// need to look up these annotations
impl Cave {
    fn new(dim: (usize, usize), x_bounds: (usize, usize), y_bounds: (usize, usize)) -> Self {
        let (width, height) = dim;
        let map = vec![vec!['.'; width]; height];
        Cave {
            map,
            width,
            height,
            x_bounds,
            y_bounds,
            sand_pos: (0, 0),
        }
    }

    // translate index input pos to array
    fn reindex(&self, pos: (usize, usize)) -> (usize, usize) {
        let (x, y) = pos;
        let trans_x = x - self.x_bounds.0 as usize;
        let trans_y = y - self.y_bounds.0 as usize;
        (trans_x, trans_y)
    }

    fn load(&mut self, xray: Vec<Vec<(usize, usize)>>) {
        self.sand_pos = self.reindex((500, 0));
        for trace in xray.iter() {
            for i in 1..trace.len() {
                let (tx, ty) = self.reindex(trace[i - 1]);
                let (hx, hy) = self.reindex(trace[i]);

                // interpolate positions between point pair to draw line
                // same x, move y
                if tx == hx {
                    let start_y = min(ty, hy);
                    let end_y = max(ty, hy);
                    for y in start_y..=end_y {
                        //self.map[height - 1 - y][hx] = '#';
                        self.map[y][hx] = '#';
                    }
                // same y, move x
                } else {
                    let start_x = min(tx, hx);
                    let end_x = max(tx, hx);
                    for x in start_x..=end_x {
                        //self.map[height - 1 - hy][x] = '#';
                        self.map[hy][x] = '#';
                    }
                }
            }
        }
    }

    fn drop_sand(&mut self) -> bool {
        // x moves:
        //      0  down
        //     -1  diag left (decrease x towards 0)
        //      1  diag right
        static TRANSFORM: [isize; 3] = [0, -1, 1];

        let (mut x, mut y) = self.sand_pos;
        if self.map[y][x] != '.' {
            // sand entry point already blocked
            return false;
        }
        'fall: loop {
            // let the spice flow
            for t in TRANSFORM {
                let dx: usize = (x as isize + t) as usize;
                let dy: usize = y + 1; // y always increases
                if dx >= self.width || dy >= self.height {
                    // sand must leave the map
                    return false;
                }
                //println!("{} {} = {} {:?}", dx, dy, self.map[dy][dx], t);
                if self.map[dy][dx] == '.' {
                    x = dx;
                    y = dy;
                    continue 'fall;
                }
            }
            // if we are here we did not continue a fall transform
            // or fall out of the map, so we must be done falling
            self.map[y][x] = 'O';
            break 'fall;
        }
        true
    }

    fn draw(&self) {
        for row in self.map.iter() {
            for cell in row.iter() {
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let capture_spec = Regex::new(r"(\d+,\d+)").unwrap();

    let mut xray: Vec<Vec<(usize, usize)>> = Vec::new();

    let mut x_bound: (usize, usize) = (usize::MAX, 0);
    let mut y_bound: (usize, usize) = (0, 0);

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let mut this_ray: Vec<(usize, usize)> = Vec::new();

        // iter over matching (x, y) co-ordinates
        for m in capture_spec.find_iter(&this_line) {
            let (x_s, y_s) = &m.as_str().split_once(',').unwrap();
            let x: usize = x_s.parse().unwrap();
            let y: usize = y_s.parse().unwrap();

            if x < x_bound.0 {
                x_bound.0 = x;
            } else if x > x_bound.1 {
                x_bound.1 = x;
            }
            if y < y_bound.0 {
                y_bound.0 = y;
            } else if y > y_bound.1 {
                y_bound.1 = y;
            }
            this_ray.push((x, y));
        }
        xray.push(this_ray);
    }

    // Expand bounds for Part 2
    let part2 = false;
    if part2 {
        x_bound.0 -= 200;
        x_bound.1 += 200;
        y_bound.1 += 2; // floor is max(y)+2
        // add line to xray describing part 2 floor
        xray.push(vec![
            (x_bound.0, y_bound.1),
            (x_bound.1, y_bound.1),
        ]);
    }

    // calculate grid dim
    let height: usize = (y_bound.1 - y_bound.0) + 1;
    let width: usize = (x_bound.1 - x_bound.0) + 1;

    let mut cave = Cave::new((width, height), x_bound, y_bound);
    cave.load(xray);
    cave.draw();
    let mut count = 0;
    while cave.drop_sand() {
        count += 1;
    }
    cave.draw();
    println!("Dropped {} sand units", count);
}
