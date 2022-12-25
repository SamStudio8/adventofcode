// Day 18
// Lava drop

use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn get_adjacent_cubes(pos: &[isize; 3]) -> Vec<[isize; 3]> {
    static TRANSFORM: [[isize; 3]; 6] = [
        [0, 0, 1],  // up
        [0, 0, -1], // down
        [-1, 0, 0], // left
        [1, 0, 0],  // right
        [0, 1, 0],  // forward
        [0, -1, 0], // backward
    ];
    TRANSFORM
        .iter()
        .map(|t| [pos[0] + t[0], pos[1] + t[1], pos[2] + t[2]])
        .collect()
}

fn count_empty_sides(pos: &[isize; 3], occupied: &HashSet<[isize; 3]>) -> usize {
    let mut empty_sides = 6;
    for adj_cube in get_adjacent_cubes(pos) {
        if occupied.contains(&adj_cube) {
            empty_sides -= 1;
        }
    }
    empty_sides
}

fn get_bounds(index: usize, occupied: &HashSet<[isize; 3]>) -> (isize, isize) {
    let bound_min = occupied.iter().map(|pos| pos[index]).min().unwrap();
    let bound_max = occupied.iter().map(|pos| pos[index]).max().unwrap();
    (bound_min - 1, bound_max + 1)
}

fn main() {
    let capture_spec = Regex::new(r"(?P<x>\d+),(?P<y>\d+),(?P<z>\d+)").unwrap();
    let mut occupied_positions: HashSet<[isize; 3]> = HashSet::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();
        let x: isize = matches["x"].parse().unwrap();
        let y: isize = matches["y"].parse().unwrap();
        let z: isize = matches["z"].parse().unwrap();
        occupied_positions.insert([x, y, z]);
    }

    // just count sides that don't have an adjacent cube
    let mut sides_open = 0;
    for cube in &occupied_positions {
        sides_open += count_empty_sides(cube, &occupied_positions);
    }
    println!("Sides open: {}", sides_open);

    // flood fill a bounding box with BFS to count exterior sides
    let (min_x, max_x) = get_bounds(0, &occupied_positions);
    let (min_y, max_y) = get_bounds(1, &occupied_positions);
    let (min_z, max_z) = get_bounds(2, &occupied_positions);
    let range_x = min_x..=max_x;
    let range_y = min_y..=max_y;
    let range_z = min_z..=max_z;

    let mut outside_sides = 0;
    let mut q: VecDeque<[isize; 3]> = VecDeque::from([[max_x, max_y, max_z]]);
    let mut visit: HashSet<[isize; 3]> = HashSet::new();

    while !q.is_empty() {
        let cube = q.pop_front().unwrap();
        if visit.contains(&cube) {
            continue;
        }
        visit.insert(cube);

        // for each adjacent cube
        // count a side if the adj cube is part of the target
        for adj_cube in get_adjacent_cubes(&cube) {
            if occupied_positions.contains(&adj_cube) {
                outside_sides += 1;
            } else {
                // queue the adj_cube
                // (but only if it does not leave the map!)
                if range_x.contains(&adj_cube[0])
                    && range_y.contains(&adj_cube[1])
                    && range_z.contains(&adj_cube[2])
                {
                    q.push_back(adj_cube);
                }
            }
        }
    }
    println!("Sides outside: {}", outside_sides);
}
