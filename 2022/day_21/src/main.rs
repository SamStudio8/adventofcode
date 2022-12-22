// Day 21
// Monkey math

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
impl std::str::FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(format!("'{}' is not a valid operator!", s)),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    op: Option<Op>,
    lhs_name: Option<String>,
    //rhs_name: Option<String>,
    lhs: Option<usize>,
    rhs: Option<usize>,
    val: Option<usize>,
}
impl Monkey {
    fn listen(&mut self, shout_name: &str, shout_val: usize) {
        if shout_name == self.lhs_name.as_ref().unwrap(){ // this feels dumb
            self.lhs = Some(shout_val);
        }
        else {
            self.rhs = Some(shout_val);
        }

        if self.can_shout() {
            self.val = Some(self._calculate_val());
        }
    }
    fn can_shout(&self) -> bool {
        self.lhs.is_some() && self.rhs.is_some()
    }
    fn _calculate_val(&self) -> usize { // fuck me that's a lot of unwrap
        match self.op.as_ref().unwrap() {
            Op::Add => self.lhs.unwrap() + self.rhs.unwrap(),
            Op::Sub => self.lhs.unwrap() - self.rhs.unwrap(),
            Op::Mul => self.lhs.unwrap() * self.rhs.unwrap(),
            Op::Div => self.lhs.unwrap() / self.rhs.unwrap(),
        }
    }
    fn adjust_shout(&mut self, preshout_name: &str, last: usize) -> usize {
        // root special cast, set the preshout to the "other" side
        if self.name == "root" {
            if self.lhs_name.as_ref().unwrap() == preshout_name {
                self.lhs = self.rhs;
            }
            else {
                self.rhs = self.lhs;
            }
            self.val = self.lhs;
            return self.val.unwrap();
        }

        let other_is_left = self.lhs_name.as_ref().unwrap() != preshout_name;
        let other = match other_is_left {
            true => self.lhs.unwrap(),
            false => self.rhs.unwrap(),
        };
        // this is where the magic happens!
        let v: usize = match (self.op.as_ref().unwrap(), other_is_left) {
            (Op::Add, _) => last - other,
            (Op::Sub, true) => other - last,
            (Op::Sub, false) => other + last,
            (Op::Mul, _) => last / other,
            (Op::Div, true) => last / other,
            (Op::Div, false) => last * other,
        };
        if other_is_left {
            self.rhs = Some(v);
        }
        else {
            self.lhs = Some(v);
        }
        self.val = Some(self._calculate_val());
        v
    }
}


fn main() {

    let mut monkey_vec: Vec<Monkey> = Vec::new();
    let mut name_to_index: HashMap<String, usize> = HashMap::new();
    let mut subscriptions: HashMap<String, Vec<String>> = HashMap::new();
    let mut messages: VecDeque<(String, usize)> = VecDeque::new();

    let stdin = io::stdin();
    for (i, line) in stdin.lock().lines().enumerate() {
        let this_line = line.unwrap();
        let this_fields: Vec<&str> = this_line.split(' ').collect();
        let monkey_name = this_fields[0].replace(':', "");

        let monkey: Monkey;
        if this_fields.len() == 2 {
            let val = Some(this_fields[1].parse().unwrap());
            // this message is ready to fire as it's already evaluated
            messages.push_back((monkey_name.to_string(), val.unwrap()));

            // val monkey
            monkey = Monkey {
                name: monkey_name.to_string(),
                op: None,
                lhs_name: None,
                //rhs_name: None,
                lhs: None,
                rhs: None,
                val,
            };
        }
        else {
            // must be a cheeky math monkey
            let lhs_name = Some(this_fields[1].to_string());
            let op: Option<Op> = Some(this_fields[2].parse().unwrap());
            let rhs_name = Some(this_fields[3].to_string());

            monkey = Monkey {
                name: monkey_name.to_string(),
                op,
                lhs_name: lhs_name.clone(),
                //rhs_name: rhs_name.clone(),
                lhs: None,
                rhs: None,
                val: None,
            };

            // map lhs/rhs entries to this monkey
            // so when the left or right monkey is updated we can look up
            // all monkeys waiting to hear about it
            subscriptions.entry(lhs_name.unwrap().clone())
                .or_default()
                .push(monkey_name.to_string());
            subscriptions.entry(rhs_name.unwrap().clone())
                .or_default()
                .push(monkey_name.to_string());
        }

        monkey_vec.push(monkey);
        name_to_index.insert(monkey_name.to_string(), i); // index name to monkey vec
    }

    while !messages.is_empty() {
        let (shout_name, shout_val) = messages.pop_front().unwrap();
        // let monkeys waiting on this shout know
        if !subscriptions.contains_key(&shout_name) {
            continue;
        }
        for listen_name in subscriptions.get(&shout_name).unwrap() {
            let listen_index = *name_to_index.get(listen_name).unwrap();
            let listen_monkey = &mut monkey_vec[listen_index];
            listen_monkey.listen(&shout_name, shout_val);

            // is this monkey ready to shout?
            if listen_monkey.can_shout() {
                messages.push_back((listen_monkey.name.to_string(), listen_monkey.val.unwrap()));
            }
        }
    }
    let root_monkey_index = *name_to_index.get("root").unwrap();
    let root_monkey = &monkey_vec[root_monkey_index];
    println!("Root monkey shouts: {}", root_monkey.val.unwrap());

    // for part two, work out the path from humn to root
    // do i actually need all this cloning?
    let mut humn_to_root = vec!["humn".to_string()];
    let mut humn_subscriptions = VecDeque::from(subscriptions.get("humn").unwrap().clone());
    while !humn_subscriptions.is_empty() {
        let listen_name = humn_subscriptions.pop_front().unwrap().clone();
        if subscriptions.contains_key(&listen_name) {
            humn_subscriptions.extend(subscriptions.get(&listen_name).unwrap().clone());
        }
        humn_to_root.push(listen_name);
    }

    // now work backwards from the root to humn and work out what the
    // monkey in the humn_to_root list needs to shout at each turn
    let mut target = 0;
    for i in 1..humn_to_root.len() {
        // the current monkey that shouts
        let shout_name = &humn_to_root[humn_to_root.len() - i];
        let shout_i = *name_to_index.get(shout_name).unwrap();
        let shout_monkey = &mut monkey_vec[shout_i];
        // the monkey that shouted into this one
        let preshout_name = &humn_to_root[humn_to_root.len() - i - 1];

        // find what the preshout money needs to shout to
        // satisfy the condition in shout_monkey
        target = shout_monkey.adjust_shout(preshout_name, target);
        shout_monkey.val.unwrap();
        //println!("{} ({}) => {:?}", preshout_name, target, shout_monkey);
    }
    println!("Humn shouts: {}", target);
}
