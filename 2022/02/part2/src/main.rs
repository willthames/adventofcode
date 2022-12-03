use std::collections::HashMap;
use std::io;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

#[derive(Copy, Clone)]
enum Strategy {
    Win,
    Lose,
    Draw,
}

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

fn choose(opponent: u8, strategy: Strategy) -> u8{
    let  win_strategy:Vec<u8> = vec![0, PAPER, SCISSORS, ROCK];
    let draw_strategy = vec![0, ROCK, PAPER, SCISSORS];
    let lose_strategy = vec![0, SCISSORS, ROCK, PAPER];

    match strategy {
        Strategy::Win => win_strategy[opponent as usize],
        Strategy::Lose => lose_strategy[opponent as usize],
        Strategy::Draw => draw_strategy[opponent as usize],
    }
}

fn main() {
    let rock_paper_scissors: HashMap<&str, u8> = HashMap::from([
        ("A", ROCK),
        ("B", PAPER),
        ("C", SCISSORS),
    ]);

    let strategy: HashMap<&str, Strategy> = HashMap::from([
        ("X", Strategy::Lose),
        ("Y", Strategy::Draw),
        ("Z", Strategy::Win),
    ]);

    let lines = io::stdin().lines();
    let mut total: u32 = 0;

    for line in lines {
        let data = line.unwrap();
        let tokens: Vec<&str> = data.split(" ").collect();
        let opponent = rock_paper_scissors[tokens[0]];
        let you = choose(opponent, strategy[tokens[1]].clone());
        // swap the tokens - score is written from the point of view of the first argument
        total += (you + score(you, opponent)) as u32;
    }

    println!("{}", total);
}