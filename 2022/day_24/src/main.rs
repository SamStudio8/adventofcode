// Day 24
// Blizzard

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    minutes: usize,
    blizzards_m: HashMap<(usize, usize), Vec<char>>,
    position: (usize, usize),
    end: (usize, usize),
    start: (usize, usize),
}
impl Map {
    fn new(width: usize, height: usize) -> Self {
        let map = vec![vec!['.'; width]; height];
        let blizzards_m = HashMap::new();
        Map { 
            map,
            width,
            height,
            minutes: 0,
            blizzards_m, // map (x,y) to [^>v<]
            position: (1,0),
            start: (1, 0),
            end: (width-2, height-1),
        }
    }
    fn load(&mut self, lines: &[String]) {
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    self.map[y][x] = c;
                }
                else if c != '.' {
                    self.blizzards_m.entry( (x,y) )
                        .or_default()
                        .push(c);
                }
            }
        }
    }
    fn draw(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if self.position == (x, y) {
                    print!("E");
                }
                else if self.blizzards_m.contains_key( &(x, y) ) {
                    let dir_vec = self.blizzards_m.get(&(x, y)).unwrap();
                    if dir_vec.len() > 1 {
                        print!("{}", dir_vec.len());
                    }
                    else {
                        print!("{}", dir_vec[0]);
                    }
                }
                else {
                    print!("{}", c);
                }
            }
            print!(" ({})", y);
            println!();
        }
    }
    fn oob(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0  || x >= self.width as isize || y >= self.height as isize
    }
    fn occupied(&self, x: usize, y: usize) -> bool {
        if (x, y) == self.start || (x, y) == self.end {
            // start and end are not occupied
            // allows us to sit on the start (or end) point to wait if needed
            return false;
        }
        if self.map[y][x] == '#' {
            // walls are occupied
            return true;
        }
        // elsewise check if snow blocks the path
        self.blizzards_m.contains_key( &(x, y) )
    }
    fn step(&mut self) {
        self.minutes += 1;
        let dir_to_idx = HashMap::from([
            ('^', 0), 
            ('>', 1),
            ('v', 2),
            ('<', 3),
        ]);
        static TRANSFORMS: [(isize, isize); 4] = [
            ( 0, -1), // ^
            ( 1,  0), // >
            ( 0,  1), // v : took me a second to work out how to write a down arrow
            (-1,  0), // <
        ];

        // i'm sure we could do something clever with the timing of the snowflakes
        // i was thinking you could have stuck with the grid and just used bits to encode flake dirs
        let mut new_blizzards_m: HashMap<(usize, usize), Vec<char>> = HashMap::new();
        for (&(x, y), dir_vec) in self.blizzards_m.iter() {
            for &flake in dir_vec.iter() {
                let d = *dir_to_idx.get(&flake).unwrap();
                let t = TRANSFORMS[d];
                let mut dx = x as isize + t.0;
                let mut dy = y as isize + t.1;
                // lol should probably clean this up but its christmas eve :shrug:
                if dx == 0 {
                    dx = self.width as isize - 2;
                }
                else if dx == self.width as isize - 1 {
                    dx = 1;
                }
                if dy == 0 {
                    dy = self.height as isize - 2;
                }
                else if dy == self.height as isize - 1 {
                    dy = 1;
                }
                new_blizzards_m.entry((dx as usize, dy as usize))
                    .or_default()
                    .push(flake);
            }
        }
        self.blizzards_m = new_blizzards_m;
    }
}

fn main() {

    // read map lines, get width/height
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

    // load map
    let mut map = Map::new(width, height);
    map.load(&stdin_lines);
    println!("***");

    static TRANSFORMS: [(isize, isize); 5] = [
        ( 0, -1), // ^
        ( 1,  0), // >
        ( 0,  1), // v : took me a second to work out how to write a down arrow
        (-1,  0), // <
        ( 0,  0), // special stand still transform
    ];

    // BFS this fucker
    let mut goal_times: Vec<usize> = Vec::new();
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

    // swap the goal from the end state to the start and back again
    // so the elf can have their snackies
    for goal in [(width-2, height-1), (1,0), (width-2, height-1)] {
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back((map.position.0, map.position.1)); // init state queue with current pos
        println!("Navigating from: ({}, {})", map.position.0, map.position.1);
        'search: loop {
            map.step();
            //map.draw();

            // initially was getting confused at how my state was getting out of sync
            // this ensures we process all the states in the current minute before
            // calling map.step to advance the map and minute counter
            let mut next_q: VecDeque<(usize, usize)> = VecDeque::new();
            while !q.is_empty() {
                let (px, py) = q.pop_front().unwrap();
                map.position = (px, py);
                if visited.contains( &(px, py, map.minutes) ) {
                    continue;
                }
                visited.insert( (px, py, map.minutes) );

                for t in TRANSFORMS {
                    let dpx = px as isize + t.0;
                    let dpy = py as isize + t.1;
                    if (dpx as usize, dpy as usize) == goal {
                        // shortcut goal step from in here to keep minutes right
                        println!("{} {}, {} {}, g={:?}, mp={:?}", px, py, dpx, dpy, goal, map.position);
                        goal_times.push(map.minutes);
                        map.position = (dpx as usize, dpy as usize);
                        println!("Reached ({}, {})@{}' after {} states", goal.0, goal.1, map.minutes, visited.len());
                        break 'search;
                    }
                    if !map.oob(dpx, dpy) && !map.occupied(dpx as usize, dpy as usize) {
                        next_q.push_back((dpx as usize, dpy as usize));
                    }
                }
            }
            q = next_q;
        }
    }

    println!("***");
    println!("Fastest route to goal: {}", goal_times[0]);
    println!("Fastest route to goal, snacks and back again: {}", map.minutes);
}
