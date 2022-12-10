// Day 2
// Rock paper scissors

use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq)]
enum Play {
    Rock=1,
    Paper=2,
    Scissors=3,
}
#[derive(Copy, Clone)] // otherwise `use of moved value` because no Copy
enum PlayResult {
    Win=6,
    Draw=3,
    Loss=0,
}

// https://stackoverflow.com/a/61948093
// https://doc.rust-lang.org/std/str/trait.FromStr.html
impl std::str::FromStr for Play {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(format!("'{}' is not a valid value!", s)),
        }
    }
}
impl std::str::FromStr for PlayResult {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(PlayResult::Loss),
            "Y" => Ok(PlayResult::Draw),
            "Z" => Ok(PlayResult::Win),
            _ => Err(format!("'{}' is not a valid value!", s)),
        }
    }
}

// https://stackoverflow.com/questions/28024373/is-there-a-way-to-print-enum-values
impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match *self {
           Play::Rock => write!(f, "Rock"),
           Play::Paper => write!(f, "Paper"),
           Play::Scissors => write!(f, "Scissors"),
       }
    }
}
impl std::fmt::Display for PlayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match *self {
           PlayResult::Win => write!(f, "WIN :)"),
           PlayResult::Loss => write!(f, "LOSS :("),
           PlayResult::Draw => write!(f, "DRAW =="),
       }
    }
}

fn lose_against(p: Play) -> Play {
    match p {
        Play::Rock => Play::Scissors,
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
    }
}
fn win_against(p: Play) -> Play {
    match p {
        Play::Scissors => Play::Rock,
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
    }
}

fn get_play(p: Play, desired_result: PlayResult) -> Play {
    match desired_result {
        PlayResult::Draw => p.clone(),
        PlayResult::Win => win_against(p),
        PlayResult::Loss => lose_against(p),
    }
}

fn main() {

    // https://stackoverflow.com/a/17546731
    let stdin = io::stdin();

    let mut score = 0;
    let mut second_score = 0;

    for line in stdin.lock().lines() {
        // unwrap allows us to implicitly handle an Option
        // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
        let this_line = line.unwrap();
        let vec: Vec<&str>= this_line.split(' ').collect();
        println!("{:?}", vec);

        let them: Play = vec[0].parse().unwrap();
        let us: Play = vec[1].parse().unwrap();

        let result: PlayResult;
        if us == them {
            // we drew
            result = PlayResult::Draw;
        }
        else if them == lose_against(us) {
            // we won
            result = PlayResult::Win;
        }
        else {
            result = PlayResult::Loss;
        }
        let this_score = result as u32 + us as u32;
        score += this_score;
        println!("1: {} : {} = {} ({} {})", them, us, result, this_score, score);

        let rigged_result: PlayResult = vec[1].parse().unwrap();
        let us: Play = get_play(them, rigged_result);
        let this_second_score = rigged_result as u32 + us as u32;
        second_score += this_second_score;
        println!("2: {} : {} = {} ({} {})", them, us, rigged_result, this_second_score, second_score);








    }
    println!("***");
    println!("Final score: {}", score);
    println!("Final final score: {}", second_score);
}
