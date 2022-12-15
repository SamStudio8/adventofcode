// Day 15 p2
// Beacons

use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;


fn manhattan(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}
fn main() {

    let stdin = io::stdin();

    let capture_spec = Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();

    //let min_x: isize = -10;
    //let max_x: isize = 30;

    // we'll build a list of all the edges and then come back and
    // compare them to each of the known sensors
    let mut edge_positions: HashSet<(isize, isize)> = HashSet::new();
    let min_edge = 0;
    let max_edge = 20;
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
        let start_y = (sensor_y - sensor_r) - 1;
        let end_y = (sensor_y + sensor_r) + 1;
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

            if y > max_edge || y < min_edge {
                continue;
            }
            if l_edge_x <= max_edge && l_edge_x >= min_edge {
                edge_positions.insert((l_edge_x, y));
            }
            if r_edge_x <= max_edge && r_edge_x >= min_edge {
                edge_positions.insert((r_edge_x, y));
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

    'out: for &edge in edge_positions.iter() {
        let (edge_x, edge_y) = edge;
        for (&sensor, &sensor_r) in sensors.iter() {
            let (sensor_x, sensor_y) = sensor;
            let d = manhattan(edge_x, edge_y, sensor_x, sensor_y);
            if d <= sensor_r {
                // inside, just move to next edge candidate
                continue 'out
            }
        }
        println!("Tuning frequency: {}", edge_x * 4000000 + edge_y);
        break;
    }
}
