// Day 7
// File system

use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {

    let stdin = io::stdin();

    let mut cwd = "".to_string();
    let mut fs: HashMap<String, u32> = HashMap::new();

    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        if this_line.starts_with('$') {
            println!("cmd> {}", this_line);
            if this_line.starts_with("$ cd") {
                let this_cd = this_line.split(' ').last().unwrap();
                if this_cd == ".." {
                    // chop end off
                    // split_inclusive would be useful here but is unstable
                    cwd = cwd.rsplit_once('/').unwrap().0.to_string();
                }
                else if this_cd == "/" {
                    // root encoded as empty string a la ["", "path", "to", "dir"]
                    cwd = "".to_string();
                }
                else {
                    cwd = format!("{}/{}", cwd, this_cd);
                }

                if !fs.contains_key(&cwd) {
                    fs.insert(cwd.to_string(), 0);
                }
                println!("{}", cwd);
            }
        }
        else {
            // if we're not reading commands we must be reading ls results
            let ls_t = this_line.split_once(' ').unwrap();
            if ls_t.0 == "dir" {
                // dont bother reading dirs, their size is 0
                // unless we cd into them and read something
                continue;
            }
            let size: u32 = ls_t.0.parse().unwrap();
            for (key, value) in fs.iter_mut() {
                if cwd.starts_with(key) {
                    // add the file size to the dir size for all dirs
                    // that match the start of the file abspath
                    *value += size;
                }
            }

            // initially did this as i couldnt work out how to iter a hashmap
            //   and update it at the same time, now i understand we can use
            //   iter_mut to get a mutable ref to value and iteract with that!
            //let mut matches: Vec<String> = Vec::new();
            //for key in fs.keys() {
            //    if cwd.starts_with(key) {
            //        matches.push(key.to_string());
            //    }
            //}
            //for key in matches {
            //    let entry = fs.entry(key).and_modify(|s| *s += size as u32);
            //}
        }
    }

    let threshold = 100_000; // need this _ trick works in rust too
    let total_used_space = *fs.get("").unwrap(); // total used space is at root
    let mut reclaim_space = 0;
    // need to call iter explicitly here
    // into_iter will allow direct access to value but takes ownership away
    //   breaking loops over fs later
    for (_key, &value) in fs.iter() {
        if value <= threshold {
            reclaim_space += value;
        }
    }
    println!("***");
    println!("Reclaim space: {}", reclaim_space);

    println!("***");
    let df = 70000000;
    let target_unused = 30000000;
    let current_free = df - total_used_space;
    let target_to_reclaim = target_unused - current_free;
    println!("Need to reclaim: {}", target_to_reclaim);

    // delete the root if there are no better options
    let mut current_candidate_name = "";
    let mut current_candidate_size = total_used_space;
    // need to use iter() manually, into_iter() is the default and takes ownership!
    for (key, &value) in fs.iter() {
        if value > target_to_reclaim && value < current_candidate_size {
            current_candidate_name = key;
            current_candidate_size = value;
        }
    }
    println!("Delete: {}", current_candidate_name);
    println!("to reclaim: {}", current_candidate_size);

}
