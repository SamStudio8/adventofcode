// Day 23
// Planting

use std::io::{self, BufRead};
use std::collections::HashMap;

struct Map {
    map: Vec<Vec<char>>, // should just be a HashMap really
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    direction: u8,
    proposed_moves: HashMap<(usize, usize), Vec<(usize, usize)>>,
}
impl Map {
    fn new(width: usize, height: usize, offset_x: usize, offset_y: usize) -> Self {
        let map = vec![vec!['.'; width]; height];
        let direction = 0; // NSWE
        Map { map, width, height, offset_x, offset_y, direction, proposed_moves: HashMap::new() }
    }
    fn load(&mut self, lines: &[String]) {
        for (y, line) in lines.iter().enumerate() {
            // line ends when it ends, but need to determine start
            for (x, c) in line.chars().enumerate() {
                self.map[y + self.offset_y][x + self.offset_x] = c;
            }
        }
    }
    fn draw(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for c in row.iter() {
                print!("{}", c);
            }
            print!(" ({})", y);
            println!();
        }
    }
    fn get_score(&self) -> usize {
        //TODO make this less bad
        let mut min_y = self.height;
        let mut max_y = 0;
        let mut min_x = self.width;
        let mut max_x = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '#' {
                    if y < min_y {
                        min_y = y;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                    if x < min_x {
                        min_x = x;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                }
            }
        }
        let mut score = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = self.map[y][x];
                if c == '.' {
                    score += 1;
                }
                print!("{}", c);
            }
            print!(" ({})", y);
            println!();
        }
        score
    }
    fn turn(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }
    fn propose_moves(&mut self) {
        // can't use iter() here?
        self.proposed_moves.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.map[y][x];
                if c == '#' {
                    // if this is an elf, get a valid move direction to move them
                    let d = self.get_move_direction(y, x);
                    self._propose_move(y, x, d); // and propose the move
                }
            }
        }
    }
    fn accept_moves(&mut self) {
        for (dest, sources) in &self.proposed_moves {
            if sources.len() == 1 {
                // only accept cases with one proposal
                let source = sources[0];
                self.map[source.0][source.1] = '.';
                self.map[dest.0][dest.1] = '#';
            }
        }
    }
    fn _propose_move(&mut self, y: usize, x: usize, d: Option<u8>) {
        if d.is_none() {
            // ignore bad moves
            return
        }
        static TRANSFORM: [(isize, isize); 4] = [
            (-1,  0), // 0 north toward 0
            ( 1,  0), // 1 south
            ( 0, -1), // 2 west toward 0
            ( 0,  1), // 3 east
        ];
        let t = TRANSFORM[d.unwrap() as usize];
        let dy = (y as isize + t.0) as usize;
        let dx = (x as isize + t.1) as usize;
        self.proposed_moves.entry( (dy,dx) )
            .or_default()
            .push( (y,x) );

    }
    fn get_move_direction(&self, y: usize, x: usize) -> Option<u8> {
        // could probably make this fn smaller
        static TRANSFORM: [(isize, isize); 8] = [
            (-1,  0), // 0 north toward 0
            (-1,  1), // 1 NE
            ( 0,  1), // 2 east
            ( 1,  1), // 3 SE
            ( 1,  0), // 4 south
            ( 1, -1), // 5 SW
            ( 0, -1), // 6 west toward 0
            (-1, -1), // 7 NW
        ];
        let propose_transforms = [
            // propose north
            [0,1,7],
            // propose south
            [3,4,5],
            // propose west
            [5,6,7],
            // propose east
            [1,2,3],
        ];

        let mut ti = self.direction;
        let mut candidate_ti = None;
        let mut occupied_dirs = 0;

        // search four compass points
        for _ in 0..4 {
            let mut this_face_clear = true;
            for t in propose_transforms[ti as usize] {
                let dy = (y as isize + TRANSFORM[t].0) as usize;
                let dx = (x as isize + TRANSFORM[t].1) as usize;
                if self.map[dy][dx] != '.' {
                    // not a valid move
                    // turn the elf and continue search
                    occupied_dirs += 1;
                    this_face_clear = false;
                }
            }
            // got through all transforms without continue
            // store this as the recommended move direction
            if candidate_ti.is_none() && this_face_clear {
                candidate_ti = Some(ti);
            }
            ti = (ti + 1) % 4; // push search to next compass point
        }

        // at least one direction was blocked, emit valid move
        if candidate_ti.is_some() && occupied_dirs > 0 {
            return candidate_ti;
        }
        // did not find a good move (or no elves in the area)
        None
    }
}

fn main() {

    let field_size = 1000; // lol just set this big enough for your data

    let stdin = io::stdin();
    let mut stdin_lines: Vec<String> = Vec::new();
    let mut width = 0;
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        if this_line.len() > width {
            width = this_line.len();
        }
        stdin_lines.push(this_line.clone());
    }
    let height = stdin_lines.len();

    let offset_x = (field_size/2) - (width/2);
    let offset_y = (field_size/2) - (height/2);
    let mut map = Map::new(field_size, field_size, offset_x, offset_y);
    map.load(&stdin_lines);
    //map.draw();
    println!("***");

    let mut rounds = 0;
    loop {
        rounds += 1;
        // propose moves
        // let's do this the dumb way for now
        map.propose_moves();
        // accept moves until we can no longer
        if map.proposed_moves.is_empty() {
            break;
        }
        map.accept_moves();
        // rotate direction
        map.turn();
        //map.draw();
        //
        if rounds == 10 {
            let score = map.get_score();
            println!("Field score at Round 10: {}", score);
        }
    }
    let score = map.get_score();
    println!("Field score: {}", score);
    println!("Move rounds required: {}", rounds);
}
