use std::collections::HashMap;
use std::io;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

fn score(first: u8, second: u8) -> u8 {
    let diff: u8;
    if first < second {
        diff = first + 3 - second;
    } else {
        diff = first - second;
    }
    match diff {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("Unexpected diff {}", diff)
    }
}

fn main() {
    let rock_paper_scissors: HashMap<&str, u8> = HashMap::from([
        ("A", ROCK),
        ("B", PAPER),
        ("C", SCISSORS),
        ("X", ROCK),
        ("Y", PAPER),
        ("Z", SCISSORS),
    ]);

    let lines = io::stdin().lines();
    let mut total: u32 = 0;

    for line in lines {
        let data = line.unwrap();
        let tokens: Vec<&str> = data.split(" ").collect();
        let you = rock_paper_scissors[tokens[1]];
        let opponent = rock_paper_scissors[tokens[0]];
        // swap the tokens - score is written from the point of view of the first argument
        total += (you + score(you, opponent)) as u32;
    }

    println!("{}", total);
}

#[cfg(test)]
mod tests;