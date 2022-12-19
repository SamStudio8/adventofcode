// Day 16
// Valves
// this is toilet and i am NOT proud of this one!

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn get_edges(adj: &Vec<Vec<bool>>, u: usize) -> Vec<usize> {
    let mut edges = Vec::new();
    for v in 0..adj.len() {
        if adj[u][v] {
            edges.push(v);
        }
    }
    edges
}

// i am proud of this bit however
fn do_bfs(adj: &Vec<Vec<bool>>, root: usize, goal: usize) -> Option<Vec<usize>> {
    let mut explored = vec![false; adj.len()];
    let mut prev: Vec<isize> = vec![-1; adj.len()];
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(root);
    explored[root] = true;
    prev[root] = root as isize;

    while !q.is_empty() {
        let u = q.pop_front()?;
        if u == goal {
            break;
        }
        for v in get_edges(adj, u) {
            if !explored[v] {
                explored[v] = true;
                prev[v] = u as isize;
                q.push_back(v);
            }
        }
    }
    // reverse prev to construct shortest path
    let mut path: Vec<usize> = Vec::new();
    let mut u = prev[goal] as usize;
    while u != root {
        path.push(u);
        u = prev[u] as usize;
    }
    path.push(u);
    Some(path)
}

struct Volcano {
    num_valves: usize,
    adj_mat: Vec<Vec<bool>>,
    distance_mat: Vec<Vec<usize>>,
    id_to_flow: HashMap<usize, usize>, // link valve numbers to flow rates
    id_to_name: HashMap<usize, String>, // link valve numbers to names
    name_to_id: HashMap<String, usize>, // gotta go forwards to go back
    init: bool,
}
impl Volcano {
    fn new() -> Volcano {
        Volcano {
            num_valves: 0,
            adj_mat: Vec::new(),
            distance_mat: Vec::new(),
            id_to_flow: HashMap::new(),
            id_to_name: HashMap::new(),
            name_to_id: HashMap::new(),
            init: false,
        }
    }
    fn add_valve(&mut self, name: &str) -> usize {
        // give this valve an index into adj_mat (if new)
        let valve_i = self.name_to_id.len();
        self.name_to_id.insert(name.to_string(), valve_i);
        self.id_to_name.insert(valve_i, name.to_string());
        self.num_valves = self.name_to_id.len();
        valve_i
    }
    fn set_valve(&mut self, name: &str, rate: usize) -> usize {
        let valve_i = *self.name_to_id.get(name).unwrap();
        self.id_to_flow.insert(valve_i, rate);
        valve_i
    }
    fn link_valves(&mut self, src: &str, dest: &str) {
        if !self.init {
            panic!("cannot link_valves before init");
        }
        let valve_i = *self.name_to_id.get(src).unwrap();
        let valve_j = *self.name_to_id.get(dest).unwrap();
        self.adj_mat[valve_i][valve_j] = true;
        self.adj_mat[valve_j][valve_i] = true;
    }
    fn init(&mut self) {
        self.adj_mat = vec![vec![false; self.num_valves]; self.num_valves];
        self.distance_mat = vec![vec![0; self.num_valves]; self.num_valves];
        self.init = true;
    }
    fn dist_valves(&mut self) {
        for i in 0..self.adj_mat.len() {
            for j in 0..i {
                let d = do_bfs(&self.adj_mat, i, j).expect("oh no");
                self.distance_mat[i][j] = d.len();
                self.distance_mat[j][i] = d.len();
            }
        }
    }
    fn print_adj(&self) {
        for row in self.adj_mat.iter() {
            for &cell in row {
                print!("{}", cell as u8);
            }
            println!();
        }
    }
    fn print_dist(&self) {
        for row in self.distance_mat.iter() {
            for cell in row {
                print!("[{:0>2}]", cell);
            }
            println!();
        }
    }
    // where the magic happens
    fn get_max_flow(
        &self,
        minutes_left: usize,
        flow_rate: usize,
        flow_total: usize,
        current_valve: usize,
        closed_valves: &HashSet<usize>,
    ) -> usize {
        //println!("M{}: {} mins left, {}ppm = {}p", 30-minutes_left, minutes_left, flow_rate, flow_total);
        let mut max_flow = 0;
        if minutes_left > 0 {
            for &valve_i in closed_valves {

                // travel_time + turn time
                let travel_time = self.distance_mat[current_valve][valve_i] + 1;
                if travel_time > minutes_left {
                    continue;
                }

                let mut new_closed_valves = closed_valves.clone();
                new_closed_valves.remove(&valve_i); // open

                let this_flow_rate = self.id_to_flow.get(&valve_i).unwrap();
                let new_flow_total = flow_rate * travel_time;

                let flow_tree = self.get_max_flow(
                    minutes_left - travel_time,
                    flow_rate + this_flow_rate,
                    new_flow_total,
                    valve_i,
                    &new_closed_valves,
                );
                if flow_tree > max_flow {
                    max_flow = flow_tree;
                }
            }
        }
        flow_total + max_flow
    }
    // where the magic happens (again)
    fn get_paths(
        &self,
        minutes_left: usize,
        current_valve: usize,
        closed_valves: &HashSet<usize>,
        current_path: Vec<usize>,
    ) -> Vec<Vec<usize>> {
        let mut my_paths: Vec<Vec<usize>> = vec![current_path.clone()];
        for &valve_i in closed_valves {
            let travel_time = self.distance_mat[current_valve][valve_i] + 1;
            if travel_time < minutes_left {
                // open the valve
                let mut new_closed_valves = closed_valves.clone();
                new_closed_valves.remove(&valve_i); // open
                // add jump to path
                let mut new_current_path = current_path.clone();
                new_current_path.push(valve_i);

                // recurse paths from this state
                my_paths.extend(self.get_paths(
                    minutes_left - travel_time,
                    valve_i,
                    &new_closed_valves,
                    new_current_path,
                ));
            }
        }
        my_paths
    }
    fn score_path(
        &self,
        path: &[usize],
        mut clock: usize,
    ) -> usize {
        let mut rate = 0;
        let mut total = 0;
        let mut current_valve = path[0];
        for &element in path.iter().skip(1) {
            let travel_time = self.distance_mat[current_valve][element] + 1;
            if travel_time >= clock {
                break;
            }
            clock -= travel_time;
            total += rate * travel_time;
            rate += *self.id_to_flow.get(&element).unwrap();
            current_valve = element;
        }
        total += clock * rate;
        total
    }
}

fn main() {
    let capture_spec = Regex::new(
        r"Valve (?P<valve>\w+) .* rate=(?P<rate>\d+); .* valve[s]? (?P<tunnels>[\w\s,]+)",
    )
    .unwrap();

    let mut volcano = Volcano::new();
    let mut lines: Vec<String> = Vec::new();

    // read lines to count valves
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();
        let matches = capture_spec.captures(&this_line).unwrap();
        let name = &matches["valve"];
        volcano.add_valve(name);
        lines.push(this_line);
    }
    volcano.init(); // make adj_mat

    // re-read the lines and link valves
    for this_line in lines {
        let matches = capture_spec.captures(&this_line).unwrap();
        let name = &matches["valve"];
        let rate: usize = matches["rate"].parse().unwrap();
        volcano.set_valve(name, rate);
        let tunnels: Vec<String> = matches["tunnels"]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        for t in tunnels {
            volcano.link_valves(name, &t);
        }
    }
    println!("***");
    volcano.print_adj();

    println!("***");
    volcano.dist_valves(); // pairwise bfs valves
    volcano.print_dist();

    // part 1
    let minutes = 30;
    let current_valve = *volcano.name_to_id.get("AA").unwrap();
    let mut closed_valves = HashSet::new();
    for (id, &flow) in volcano.id_to_flow.iter() {
        if flow > 0 {
            closed_valves.insert(*id);
        }
    }
    println!("***");
    let max_pressure = volcano.get_max_flow(minutes, 0, 0, current_valve, &closed_valves);
    println!(
        "Max pressure released after {} minutes: {}",
        minutes, max_pressure
    );

    // re-close valves for part 2
    let minutes = 26;
    let mut closed_valves = HashSet::new();
    for (id, &flow) in volcano.id_to_flow.iter() {
        if flow > 0 {
            closed_valves.insert(*id);
        }
    }
    let current_valve = *volcano.name_to_id.get("AA").unwrap();
    let current_path: Vec<usize> = vec![current_valve]; // push current_valve you fucking idiot
    let mut part_2_paths: Vec<(Vec<usize>, usize)> = volcano.get_paths(
        minutes,
        current_valve,
        &closed_valves,
        current_path,
    ).iter().map(|path| (path.clone(), volcano.score_path(path, minutes))).collect(); // cloned here because i dont know borrowing works

    part_2_paths.sort_by_key(|p| p.1);
    part_2_paths.reverse();
    println!("***");
    println!("Enumerated {} paths, finding best pair...", part_2_paths.len());

    let mut best = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    for i in 0..part_2_paths.len() {
        let (path_i, score_i) = &part_2_paths[i];
        'path: for (j, (path_j, score_j)) in part_2_paths.iter().enumerate().skip(i) {
            if score_i + score_j <= best {
                break; // the path list is sorted so we can't do better than this
            } else {
                for &pj in path_j.iter() {
                    if path_i.contains(&pj) && pj != current_valve {
                        // check current_valve you fucking idiot
                        continue 'path;
                    }
                }

                best = score_i + score_j;
                println!("- Interim best {}", best);
                best_i = i;
                best_j = j;
            }
        }
    }
    println!("Max pressure released after {} minutes by Sam and Nelly: {}", minutes, best);
    println!(
        "Sam  : {:?}",
        part_2_paths[best_i].0
            .iter()
            .map(|p| volcano.id_to_name.get(p).unwrap())
            .collect::<Vec<&String>>()
    );
    println!(
        "Nelly: {:?}",
        part_2_paths[best_j].0
            .iter()
            .map(|p| volcano.id_to_name.get(p).unwrap())
            .collect::<Vec<&String>>()
    );
}
