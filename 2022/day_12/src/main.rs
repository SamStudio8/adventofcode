// Day 12
// Hill climb

use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::io::{self, BufRead};

struct Map {
    w: isize,
    h: isize,
    heights: Vec<Vec<usize>>,
    visited: Vec<Vec<bool>>,
    distances: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
}
impl Map {
    // the implementation block
    fn new(width: usize, height: usize) -> Map {
        let heights = vec![vec![0; width]; height];
        let visited = vec![vec![false; width]; height];
        let distances = vec![vec![usize::MAX; width]; height];
        Map {
            w: width as isize,
            h: height as isize,
            heights,
            visited,
            distances,
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn populate(&mut self, lines: &[String]) {
        for (x, line) in lines.iter().enumerate() {
            for (y, c) in line.chars().enumerate() {
                // for some reason to_digit doesn't work here even though we've done this before!
                let cell_height = match c {
                    'S' => 0,
                    'E' => 25,
                    _ => (c as u8) - 97,
                };
                if c == 'S' {
                    self.start = (x, y);
                } else if c == 'E' {
                    self.end = (x, y);
                }
                self.heights[x][y] = cell_height as usize;
            }
        }
    }

    fn get_neighbours(&self, pos: (usize, usize)) -> HashSet<(usize, usize)> {
        static TRANSFORM: [(isize, isize); 4] = [
            (-1, 0), // left
            (0, 1),  // up
            (1, 0),  // right
            (0, -1), // down
        ];

        let (x, y) = pos;
        let mut res: HashSet<(usize, usize)> = HashSet::new();
        let src_height = self.heights[x][y] as isize;
        for t in TRANSFORM {
            let dx: isize = x as isize + t.0;
            let dy: isize = y as isize + t.1;

            // check point in grid
            if dx >= 0 && dx < self.h && dy >= 0 && dy < self.w {
                // can't revisit node
                if self.visited[dx as usize][dy as usize] {
                    continue;
                }

                // can't move up more than one level
                let dest_height = self.heights[dx as usize][dy as usize] as isize;
                //if (dest_height - src_height) > 1 {
                if (src_height - dest_height) > 1 {
                    continue;
                }

                // rack em up
                res.insert((dx as usize, dy as usize));
            }
        }
        res
    }

    fn get_coords_at_height(&self, height: usize) -> HashSet<(usize, usize)> {
        let mut res: HashSet<(usize, usize)> = HashSet::new();
        for (x, row) in self.heights.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                // if cell starts at flat, this is a candidate
                if cell == height {
                    res.insert((x, y));
                }
            }
        }
        res
    }
}

fn main() {
    let stdin = io::stdin();

    // read lines to make grid
    let mut map_lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        map_lines.push(line.unwrap().clone());
    }

    // determine grid size
    // ruin nice code with isize nonsense
    println!("*** Grid");
    let mut iwidth: isize = 0;
    let iheight: isize = map_lines.len() as isize;
    for (i, line) in map_lines.iter().enumerate() {
        if i == 0 {
            iwidth = line.len() as isize;
        }
        println!("{}", line);
    }

    // construct grid
    let mut map: Map = Map::new(iwidth as usize, iheight as usize);
    map.populate(&map_lines);

    println!("*** Heights");
    for row in map.heights.iter() {
        for res in row.iter() {
            print!("[{:0>3}]", res);
        }
        println!();
    }

    let start_pos = map.end; // start from the end
    let mut to_visit: PriorityQueue<(usize, usize), isize> = PriorityQueue::new();
    to_visit.push(start_pos, 0); // begin search by visiting start pos
    map.distances[start_pos.0][start_pos.1] = 0; // distance to start is 0

    while !to_visit.is_empty() {
        // pop the lowest distance node
        let curr_pos = to_visit.pop().unwrap().0;
        let (curr_x, curr_y) = curr_pos;
        map.visited[curr_x][curr_y] = true;

        // update distances for candidate neighbours
        for neighbour in map.get_neighbours(curr_pos) {
            let (nx, ny) = neighbour;
            let d = map.distances[curr_x][curr_y] + 1;
            if d < map.distances[nx][ny] {
                map.distances[nx][ny] = d;
            }
            // push to priority queue with inverted priority
            to_visit.push(neighbour, -(d as isize));
        }
    }

    println!("*** Visited");
    for row in map.visited.iter() {
        for res in row.iter() {
            let c = match res {
                true => '#',
                false => '.',
            };
            print!("{}", c);
        }
        println!();
    }

    // determine smallest path
    let mut lowest_dist = usize::MAX;
    let mut lowest_start = map.end;
    for pos in map.get_coords_at_height(0) {
        let (x, y) = pos;
        let d = map.distances[x][y];
        if d < lowest_dist {
            lowest_dist = d;
            lowest_start = pos;
        }
    }

    println!("*** Distances");
    for row in map.distances.iter() {
        for &res in row.iter() {
            if res == usize::MAX {
                print!("[---]");
            }
            else {
                print!("[{:0>3}]", res);
            }
        }
        println!();
    }


    println!("***");
    let (x, y) = map.start;
    println!(
        "Distance {}, starting hike at {:?}",
        map.distances[x][y], map.start
    );
    println!(
        "Distance {}, starting hike at {:?}",
        lowest_dist, lowest_start
    );
}
