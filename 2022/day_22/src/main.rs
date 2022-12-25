// Day 22
// Cube

use regex::Regex;
use std::io::{self, BufRead};

struct Map {
    map: Vec<Vec<char>>,
    x_active_ranges: Vec<(isize, isize)>,
    y_active_ranges: Vec<(usize, usize)>,
    width: usize,
    height: usize,
    face: u8,
    location: (usize, usize),
    is_cube: bool,
    dim: isize,
}
impl Map {
    fn new(width: usize, height: usize) -> Self {
        let map = vec![vec![' '; width]; height];
        let x_active_ranges = Vec::new();
        let y_active_ranges = Vec::new();
        let face = 0; // start facing up as we'll turn right on move 1
        let location = (0, 0);
        Map {
            map,
            x_active_ranges,
            y_active_ranges,
            width,
            height,
            face,
            location,
            is_cube: true,
            dim: 50,
        }
    }
    fn load(&mut self, lines: &[String]) {
        for (y, line) in lines.iter().enumerate() {
            // line ends when it ends, but need to determine start
            let mut range: (isize, isize) = (isize::MAX, (line.len() as isize) - 1);
            for (x, c) in line.chars().enumerate() {
                if c == ' ' {
                    continue;
                }
                if c != ' ' && range.0 == isize::MAX {
                    range.0 = x as isize;
                }
                self.map[y][x] = c;
                //self.map[y][x] = '.'; // testing at its finest
            }
            self.x_active_ranges.push(range);
        }

        // gross hack to make folding over columns easier
        // re-read map to get y ranges
        let mut ranges = Vec::new();
        for _ in 0..self.width {
            ranges.push((usize::MAX, self.height - 1));
        }
        for (y, row) in self.map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c != ' ' && ranges[x].0 == usize::MAX {
                    ranges[x].0 = y;
                } else if c == ' ' && ranges[x].0 < usize::MAX && ranges[x].1 == self.height - 1 {
                    // todo clean
                    ranges[x].1 = y - 1;
                }
            }
        }
        self.y_active_ranges = ranges;

        // set start
        self.location = self._get_start();
    }
    fn draw(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if y == self.location.0 && x == self.location.1 {
                    print!("o");
                } else {
                    print!("{}", c);
                }
            }
            print!(" {:?}", self.x_active_ranges[y]);
            println!();
        }
    }
    fn _get_start(&self) -> (usize, usize) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '.' {
                    return (y, x);
                }
            }
        }
        (usize::MAX, usize::MAX)
    }
    fn turn(&mut self, direction: char) {
        if direction == 'R' {
            self.face = (self.face + 1) % 4;
        } else if direction == 'L' {
            self.face = (self.face as isize - 1).rem_euclid(4) as u8; // rem_euclid strikes again
        } else {
            panic!("cannot turn direction {}", direction);
        }
    }
    fn walk(&mut self, mut paces: usize) {
        static TRANSFORMS: [(isize, isize); 4] = [
            (-1, 0), // up towards 0
            (0, 1),
            (1, 0),
            (0, -1),
        ];
        let (y, x) = self.location;
        let mut y = y as isize;
        let mut x = x as isize;
        while paces > 0 {
            let mut dy = y + TRANSFORMS[self.face as usize].0;
            let mut dx = x + TRANSFORMS[self.face as usize].1;

            // lets current the current face or something i dont know
            let current_face = self.get_face_for_position(y, x);

            let x_range = self.x_active_ranges[y as usize];
            let y_range = self.y_active_ranges[x as usize];

            let mut turns: Vec<char> = Vec::new();

            if !self.is_cube {
                // if moving in x-axis
                if dx != x as isize {
                    if dx < x_range.0 {
                        // wrap left to right
                        dx = x_range.1;
                    } else if dx > x_range.1 {
                        // wrap right to left
                        dx = x_range.0;
                    }
                }
                // fucking isize again, TODO tidy this like x
                if dy != y as isize {
                    if dy < y_range.0 as isize {
                        // wrap top to bottom
                        dy = y_range.1 as isize;
                    } else if dy > y_range.1 as isize {
                        // wrap bottom to top
                        dy = y_range.0 as isize;
                    }
                }
            } else {
                // translate across bounds
                // delicious (now in rustfmt flavour!)
                if current_face == 'a' {
                    if dy < 0 {
                        // AF
                        dy = (3 * self.dim) + (x % self.dim);
                        dx = 0;
                        turns.push('R');
                    } else if dx < x_range.0 {
                        // AD
                        dy = (3 * self.dim) - (y + 1);
                        dx = 0;
                        turns.push('R');
                        turns.push('R');
                    }
                } else if current_face == 'b' {
                    if dy < 0 {
                        // BF
                        dy = (4 * self.dim) - 1;
                        dx = x % self.dim;
                    } else if dx > x_range.1 {
                        // BE
                        dy = (3 * self.dim) - (y + 1);
                        dx = (2 * self.dim) - 1;
                        turns.push('L');
                        turns.push('L');
                    } else if dy > y_range.1 as isize {
                        // BC
                        dy = self.dim + (x % self.dim);
                        dx = (2 * self.dim) - 1;
                        turns.push('R');
                    }
                } else if current_face == 'c' {
                    if dx < x_range.0 {
                        // CD
                        dy = 2 * self.dim;
                        dx = y % self.dim;
                        turns.push('L');
                    } else if dx > x_range.1 {
                        // CB
                        dy = self.dim - 1;
                        dx = (2 * self.dim) + (y % self.dim);
                        turns.push('L');
                    }
                } else if current_face == 'd' {
                    if dx < 0 {
                        // DA
                        dy = self.dim - ((y % self.dim) + 1);
                        dx = self.dim;
                        turns.push('R');
                        turns.push('R');
                    } else if dy < y_range.0 as isize {
                        // DC
                        dy = self.dim + x;
                        dx = self.dim;
                        turns.push('R');
                    }
                } else if current_face == 'e' {
                    if dx > x_range.1 {
                        // EB
                        dy = self.dim - ((y % self.dim) + 1);
                        dx = (3 * self.dim) - 1;
                        turns.push('R');
                        turns.push('R');
                    } else if dy > y_range.1 as isize {
                        // EF
                        dy = (3 * self.dim) + (x % self.dim);
                        dx = self.dim - 1;
                        turns.push('R');
                    }
                } else if current_face == 'f' {
                    if dx < 0 {
                        // FA
                        dy = 0;
                        dx = self.dim + (y % self.dim);
                        turns.push('L');
                    } else if dy > y_range.1 as isize {
                        // FB
                        dy = 0;
                        dx = (2 * self.dim) + x;
                    } else if dx > x_range.1 {
                        // FE
                        dy = (3 * self.dim) - 1;
                        dx = self.dim + (y % self.dim);
                        turns.push('L');
                    }
                }
            }

            // can move, apply the change to x,y
            if self.map[dy as usize][dx as usize] == '.' {
                x = dx;
                y = dy;
                self.location = (y as usize, x as usize);
                // if the move was accepted, apply any queued turns
                for c in turns {
                    self.turn(c);
                }
            }
            paces -= 1;
        }
    }
    fn get_face_for_position(&self, y: isize, x: isize) -> char {
        // lol fuck this i don't care
        if y < 50 {
            if x < 50 {
                panic!("invalid cordinate");
            } else if x < 100 {
                'a'
            } else if x < 150 {
                'b'
            } else {
                panic!("invalid cordinate");
            }
        } else if y < 100 {
            if x < 50 {
                panic!("invalid cordinate");
            } else if x < 100 {
                'c'
            } else {
                panic!("invalid cordinate");
            }
        } else if y < 150 {
            if x < 0 {
                panic!("invalid cordinate");
            } else if x < 50 {
                'd'
            } else if x < 100 {
                'e'
            } else {
                panic!("invalid cordinate");
            }
        } else if y < 200 {
            if x < 0 {
                panic!("invalid cordinate");
            } else if x < 50 {
                'f'
            } else {
                panic!("invalid cordinate");
            }
        } else {
            panic!("invalid cordinate");
        }
    }
    fn get_password(&self) -> usize {
        let (row, col) = self.location;
        let row_score = 1000 * (row + 1);
        let col_score = 4 * (col + 1);
        let face_score = (self.face as isize - 1).rem_euclid(4);
        row_score + col_score + face_score as usize
    }
}

fn main() {
    let stdin = io::stdin();
    let capture_spec = Regex::new(r"(?P<face>L|R)(?P<dist>\d+)").unwrap();

    let mut stdin_lines: Vec<String> = Vec::new();
    let mut instruction = String::from("R"); // inject first move for nicer regex
    let mut read_instruction = false;
    let mut width = 0;
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        if this_line.is_empty() {
            read_instruction = true;
            continue;
        }
        if read_instruction {
            instruction.push_str(&this_line);
            break;
        }
        if this_line.len() > width {
            width = this_line.len();
        }
        stdin_lines.push(this_line.clone());
    }
    let height = stdin_lines.len();

    // read instructions to vec
    let mut instructions_v = Vec::new();
    for capture in capture_spec.captures_iter(&instruction) {
        let face = capture["face"].to_string().chars().next().unwrap();
        let dist = &capture["dist"];
        instructions_v.push((face, dist.parse::<usize>().unwrap()));
    }

    // this is quite a nice top level api for such an awful day
    let mut map = Map::new(width, height);
    map.load(&stdin_lines);
    map.draw();
    for (turn_face, walk_dist) in instructions_v {
        //println!("Start @{:?}", map.location);
        map.turn(turn_face);
        map.walk(walk_dist);
        //map.draw();
        //println!("End @{:?}", map.location);
    }
    println!("Password: {}", map.get_password());
}
