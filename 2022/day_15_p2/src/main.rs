// Day 15 p2
// Beacons

use std::io::{self, BufRead};
use regex::Regex;
use std::cmp::{min, max};
use std::collections::HashMap;
//use rustc_hash::{FxHashSet as HashSet};


fn manhattan(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {

    let stdin = io::stdin();

    let capture_spec = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

    //let min_x: isize = -10;
    //let max_x: isize = 30;

    // we'll build a list of all the sensor area edges and then come back
    // and compare their distance to each of the known sensors to find
    // the one that is out of range
    // use a Vec instead of HashSet because it's quicker to check a few
    // duplicates than take care of hashing millions of tuples (10s to <1s)
    let mut edge_positions: Vec<(isize, isize)> = Vec::new();
    // even FxHashSet was slower
    // https://nnethercote.github.io/perf-book/hashing.html
    // let mut edge_positions: HashSet<(isize, isize)> = HashSet::default();

    let min_edge = 0;
    //let max_edge = 20;
    let max_edge = 4000000;

    let mut sensors: HashMap<(isize, isize), isize> = HashMap::new();

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();
        let sensor_x: isize = matches["sensor_x"].parse().unwrap();
        let sensor_y: isize = matches["sensor_y"].parse().unwrap();
        let beacon_x: isize = matches["beacon_x"].parse().unwrap();
        let beacon_y: isize = matches["beacon_y"].parse().unwrap();

        let sensor_r = manhattan(sensor_x, sensor_y, beacon_x, beacon_y);
        sensors.insert( (sensor_x, sensor_y), sensor_r );

        // get all edge positions immediately outside the sensor area
        //    0      sensor_y - sensor_r - 1
        //   0#0
        //  0###0
        // 0##S##0   sensor_y
        //  0###0
        //   0#0
        //    0      sensor_y + sensor_r + 1
        // clip y-axis to [min_edge, max_edge]
        let start_y = max((sensor_y - sensor_r) - 1, min_edge);
        let end_y = min((sensor_y + sensor_r) + 1, max_edge);
        println!(
            "Sensor ({}, {}) detects Beacon ({}, {}) with r={}",
            sensor_x, sensor_y, beacon_x, beacon_y, sensor_r,
        );

        for y in start_y..=end_y {
            //let mut this_scan = vec!['.'; ((max_x-min_x)as usize)+1];

            let y_to_sensor = (sensor_y - y).abs();
            let start_x = sensor_x - (sensor_r - y_to_sensor);
            let end_x = sensor_x + (sensor_r - y_to_sensor);

            let l_edge_x = start_x - 1;
            let r_edge_x = end_x + 1;

            if l_edge_x <= max_edge && l_edge_x >= min_edge {
                edge_positions.push((l_edge_x, y));
            }
            if r_edge_x <= max_edge && r_edge_x >= min_edge {
                edge_positions.push((r_edge_x, y));
            }

            //let arr_x = l_edge_x + min_x.abs();
            //this_scan[arr_x as usize] = 'O';
            //let arr_x = r_edge_x + min_x.abs();
            //this_scan[arr_x as usize] = 'O';

            //for x in start_x..=end_x {
            //    let arr_x = x + min_x.abs();
            //    if arr_x < 0 {
            //        continue;
            //    }
            //    if arr_x as usize >= this_scan.len() {
            //        continue;
            //   }
            //    this_scan[arr_x as usize] = '#';
            //}

            //for c in this_scan {
            //    print!("{}", c);
            //}
            //println!();
        }
    }

    println!("Tuning...");
    let mut count: usize = 0;
    'out: for &edge in edge_positions.iter() {
        let (edge_x, edge_y) = edge;
        for (&sensor, &sensor_r) in sensors.iter() {
            let (sensor_x, sensor_y) = sensor;
            count += 1;
            if manhattan(edge_x, edge_y, sensor_x, sensor_y) <= sensor_r {
                // cell inside this sensor's range
                // move outer loop to next edge candidate
                continue 'out
            }
        }
        // searched all sensors and this cell is still out of range
        // it is THE ONE
        println!("Tuned cells: {}", count);
        println!("Tuning frequency: {}", edge_x * 4000000 + edge_y);
        break;
    }
}
