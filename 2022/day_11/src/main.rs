// Day 11
// Monkey business

use std::io::{self, BufRead};
use std::collections::VecDeque;
use regex::Regex;

#[derive(Copy, Clone)]
#[derive(Debug)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug)]
struct Monkey {
    name: u8,
    items: VecDeque<u64>,
    op: Op,
    op_v: u64,
    op_v_old: bool, // set to true to ignore op_v and use old
    test_div: u64,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

// advent of text parsing
fn build_monkey(monkey_lines: &Vec<String>) -> Monkey {
    println!("{:?}", monkey_lines);
    let monkey_string = monkey_lines.join("\n");
    let capture_spec = Regex::new(r"Monkey (?P<name>\d+):
[\s]*Starting items: (?P<items>[\d, ]+)
[\s]*Operation: new = old (?P<op>[\+\*]) (?P<op_val>\d+|old)
[\s]*Test: divisible by (?P<test_div>\d+)
[\s]*If true: throw to monkey (?P<test_true>\d)
[\s]*If false: throw to monkey (?P<test_false>\d)").unwrap();
    let matches = capture_spec.captures(&monkey_string).unwrap();

    // leaving this here for posterity
    // let items = monkey_lines[1].split(':').last().unwrap().trim().to_string().split(", ").map(|x| x.parse().unwrap()).collect();
    let name = matches["name"].parse().unwrap();
    let items = matches["items"].split(", ").map(|x| x.parse().unwrap()).collect();
    let op = match &matches["op"] {
        "+" => Op::Add,
        "*" => Op::Mult,
        _ => panic!("Bad op!")
    };
    let op_v_old = &matches["op_val"] == "old";
    let op_v = if op_v_old {
        0
    }
    else {
        matches["op_val"].parse().unwrap()
    };
    let test_div = matches["test_div"].parse().unwrap();
    let test_true = matches["test_true"].parse().unwrap();
    let test_false = matches["test_false"].parse().unwrap();
    let inspections = 0;
    Monkey { name, items, op, op_v, op_v_old, test_div, test_true, test_false, inspections }
}

fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_lines: Vec<String> = Vec::new();

    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        if this_line.starts_with("Monkey") && !monkey_lines.is_empty() {
            monkeys.push(build_monkey(&monkey_lines));
            monkey_lines = Vec::new();
        }
        monkey_lines.push(this_line.trim().to_string().clone());
    }
    monkeys.push(build_monkey(&monkey_lines));

    let mut big_worry = 1;
    for m in monkeys.iter() {
        println!("{:?}", m);
        big_worry *= m.test_div;
    }
    println!("Big worry {}", big_worry);

    // begin torment
    //let num_rounds = 10_000;
    let num_rounds = 20;
    for _ in 0..num_rounds {
        for monkey_num in 0..monkeys.len() {

            let m = &monkeys[monkey_num];
            let number_to_inspect = m.items.len();

            for _ in 0..number_to_inspect {
                let mut worry_level: u64;

                // reborrow m as mutable to pop
                let m = &mut monkeys[monkey_num];

                // get level of concern
                worry_level = m.items.pop_front().unwrap();
                println!("Monkey {} inspects an item with a worry level of {}", monkey_num, worry_level);
                m.inspections += 1;
                let op_v = if m.op_v_old {
                    worry_level
                }
                else {
                    m.op_v
                };

                // mod worry level
                worry_level = match m.op {
                    Op::Add => worry_level + op_v,
                    Op::Mult => worry_level * op_v,
                };
                println!("  Worry level is modified to {}", worry_level);

                // apply test
                //worry_level /= 3; // relief
                worry_level %= big_worry;
                println!("  Worry level is reduced to {}", worry_level);
                let throw_to = if worry_level % m.test_div == 0 {
                    m.test_true
                }
                else {
                    m.test_false
                };

                monkeys[throw_to].items.push_back(worry_level);
                println!("  Item with worry level {} is thrown to Monkey {}", worry_level, throw_to);
            }
        }
    }
    println!("***");
    for m in monkeys.iter() {
        println!("Monkey {} inspected {} items", m.name, m.inspections);
    }

    println!("***");
    let mut inspection_count: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspection_count.sort();
    let top_two = inspection_count.split_off(inspection_count.len() - 2);
    let res = top_two[0] * top_two[1];
    println!("Monkey business: {}", res);
}
