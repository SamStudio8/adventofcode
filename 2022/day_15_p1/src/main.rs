// Day 15
// Beacons

use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashSet;
use bit_vec::BitVec;


fn manhattan(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}
fn main() {

    let stdin = io::stdin();

    let capture_spec = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

    let min_x: isize = -5_000_000;
    let target_y = 2_000_000;

    // let mut blocked: HashSet<isize> = HashSet::new(); // a bit "slow" for this use case
    let mut bv = BitVec::from_elem(10_000_000, false); // just ask for loads, bits are tiny
    let mut beacons_on_target: HashSet<isize> = HashSet::new();

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();
        let sensor_x: isize = matches["sensor_x"].parse().unwrap();
        let sensor_y: isize = matches["sensor_y"].parse().unwrap();
        let beacon_x: isize = matches["beacon_x"].parse().unwrap();
        let beacon_y: isize = matches["beacon_y"].parse().unwrap();

        let sensor_r = manhattan(sensor_x, sensor_y, beacon_x, beacon_y);
        let y_to_sensor = (target_y - sensor_y).abs();

        // if y-dist between target and sensor is larger than the
        // sensor radius then we're outside the signal range
        if y_to_sensor > sensor_r {
            continue;
        }

        println!(
            "Sensor ({}, {}) detects Beacon ({}, {}) with r={}",
            sensor_x, sensor_y, beacon_x, beacon_y, sensor_r,
        );

        let start_x = sensor_x - (sensor_r - y_to_sensor);
        let end_x = sensor_x + (sensor_r - y_to_sensor);
        for x in start_x..=end_x {
            let arr_x = x + min_x.abs();
            bv.set(arr_x as usize, true);
            //blocked.insert(x);
        }
        // account for known beacons on the target
        if beacon_y == target_y {
            beacons_on_target.insert(beacon_x);
        }
    }
    //println!("Blocked tiles at y={}: {}", target_y, blocked.len());
    let count = bv.iter().filter(|x| *x).count() - beacons_on_target.len();
    println!("Blocked tiles at y={}: {}", target_y, count);
}
