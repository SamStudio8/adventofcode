// Day 13
// List distress

use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

// fuck me this was harder than i was expecting
impl Packet {
    // look at me using a &str for a change
    fn new(data: &str) -> Self {
        // base case, we've reached a single value
        if !data.starts_with('[') {
            return Self::Number(data.parse().unwrap())
        }

        let mut val_buff = String::new(); // will store latest number of latest [...] block
        let mut values = Vec::new(); // will store list of Packet at depth 0
        let mut depth = 0;

        // strip the prefix to make it easier to discern between
        // the top level and any other level
        let data = data
            .strip_prefix('[').unwrap()
            .strip_suffix(']').unwrap();
        for c in data.chars() {
            if c == '[' {
                depth += 1;
            }
            else if c == ']' {
                depth -= 1;
            }
            // either we've returned to depth 0 from a nest
            // or we've read a numeric value from depth 0
            if c == ',' && depth == 0 {
                values.push(Self::new(&val_buff));
                val_buff.clear();
            }
            else {
                val_buff.push(c);
            }
        }
        // last
        if !val_buff.is_empty() {
            values.push( Self::new(&val_buff) );
        }
        Self::List(values)
    }
}

// https://doc.rust-lang.org/std/cmp/trait.Ord.html#how-can-i-implement-ord
// requires PartialOrd, Ord and PartialEq
impl PartialOrd for Packet {
    // partial_cmp just calls cmp
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// this is where the magic happens
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        // self is left, right is other
        match (self, other) {
            // both numbers
            (Packet::Number(left), Packet::Number(right)) => {
                // trivially delegate to numeric comparator
                left.cmp(right)
            },
            // left int
            (&Packet::Number(left), right) => {
                let left_as_list = Packet::List(vec![Packet::Number(left)]);
                left_as_list.cmp(right)
            },
            // right int
            (left, &Packet::Number(right)) => {
                let right_as_list = Packet::List(vec![Packet::Number(right)]);
                left.cmp(&right_as_list)
            },
            // finally, both as list
            (Packet::List(left), Packet::List(right)) => {
                for (l, r) in left.iter().zip(right) {
                    let res = l.cmp(r);
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                // reached end of left without making a decision
                // determine ordering on length (longer left == greater == out of order)
                // or both lists empty
                left.len().cmp(&right.len())
            },
        }
    }
}

fn main() {

    let stdin = io::stdin();

    let mut line_pairs: Vec<(String, String)> = Vec::new();

    let mut s1: String = String::new();
    let mut s2: String = String::new();
    for line in stdin.lock().lines() {
        let this_line = line.unwrap();

        if this_line.is_empty() {
            line_pairs.push((s1, s2));
            s1 = "".to_string();
            s2 = "".to_string();
        }
        if s1.is_empty() {
            s1 = this_line.clone();
        }
        else {
            s2 = this_line.clone();
        }
    }
    line_pairs.push((s1, s2));

    // for part 2, count how many entries in pairs are smaller than each divider
    // i've Derived Hash which seems to be fine?
    let mut dividers: HashMap<Packet, usize> = HashMap::new();
    dividers.insert(Packet::new("[[2]]"), 1); // start count at 1 for offset
    dividers.insert(Packet::new("[[6]]"), 2); // [[2]] is smaller than [[6]] lol

    // let's also trying sorting
    let mut packets: Vec<Packet> = Vec::new();
    packets.push(Packet::new("[[2]]"));
    packets.push(Packet::new("[[6]]"));

    let mut pair_sum = 0;
    for (pair_i, pair) in line_pairs.iter().enumerate() {
        let (s1, s2) = pair;
        let left = Packet::new(s1);
        let right = Packet::new(s2);

        if left.cmp(&right) == Ordering::Less {
            pair_sum += pair_i + 1;
        }

        for (divider, value) in dividers.iter_mut() {
            if left.cmp(divider) == Ordering::Less {
                *value += 1;
            }
            if right.cmp(divider) == Ordering::Less {
                *value += 1;
            }
        }
        packets.push(left);
        packets.push(right);
    }
    println!("Sorted pairs: {}", pair_sum);

    // Decoder with count
    let mut decoder_key = 1;
    for (divider, value) in dividers.iter() {
        println!("{:?}={}", divider, value);
        decoder_key *= value;
    }

    // Decoder with sort
    packets.sort();
    println!("Decoder key: {}", decoder_key);
    let mut decoder_key = 1;
    for (divider, _value) in dividers.iter() {
        decoder_key *= packets.iter().position(|p| p == divider).unwrap() + 1;
    }
    println!("Decoder key: {}", decoder_key);


}
