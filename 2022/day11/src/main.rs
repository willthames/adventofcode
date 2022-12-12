
use std::{io, collections::LinkedList};

#[derive(Clone, Copy)]
struct Monkey {
    test: i64,
    truetest: usize,
    falsetest: usize,
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Times,
    Plus,
    Minus,
    DivideBy,
} 

fn apply(lhs: i64, op: Operation, rhs: i64) -> i64 {
    // println!("{} {:?} {}", lhs, op, rhs);
    match op {
        Operation::Times => lhs * rhs,
        Operation::Plus => lhs + rhs,
        Operation::Minus => lhs - rhs,
        Operation::DivideBy => lhs / rhs,
    }
}

fn applyself(lhs: i64, op: Operation) -> i64 {
    return apply(lhs, op, lhs);
}

fn operation(_lhs: &str, opstr: &str, rhs: &str) -> Box<dyn Fn(i64) -> i64> {
    let op = match opstr {
        "+" => Operation::Plus,
        "-" => Operation::Minus,
        "*" => Operation::Times,
        "/" => Operation::DivideBy,
        _ => panic!("Unexpected operation {}", opstr)
    };
    if rhs == "old" { 
        Box::new(
            move |x: i64| -> i64 {
                  applyself(x, op)
            }
        )
    } else {
        let val = rhs.parse::<i64>().unwrap();
        Box::new(
            move |x: i64| -> i64 {        
                apply(x, op, val) 
            }
        )
    }
}

#[allow(dead_code)]
fn show(monkeyitems: LinkedList<i64>) {
    for item in monkeyitems {
        print!("{}, ", item);
    }
    println!("");
}

#[allow(dead_code)]
fn part1(monkeys: Vec<Monkey>, monkeyfns: Vec<Box<dyn Fn(i64) -> i64>>, mut monkeyitems: Vec<LinkedList<i64>>) -> i64 {

    let mut monkeyctr = vec![0; monkeys.len()];
    for _ in 0..20 {
        for (mindex, monkey) in monkeys.iter().enumerate() {
            for item in monkeyitems[mindex].clone() {
                monkeyctr[mindex] += 1;
                let worry = (monkeyfns[mindex])(item) / 3;
                if worry % monkey.test == 0 {
                    monkeyitems[monkey.truetest].push_back(worry);
                    println!("Monkey {} gives {} to {}", mindex, worry, monkey.truetest);
                } else {
                    monkeyitems[monkey.falsetest].push_back(worry);
                    println!("Monkey {} gives {} to {}", mindex, worry, monkey.falsetest);
                }
            }
            monkeyitems[mindex] = LinkedList::new();
        }
        for idx in 0..monkeyitems.len() {
            print!("{}: ", idx);
            show(monkeyitems[idx].clone());
        }
    }
    monkeyctr.sort_by(|a, b| b.cmp(a));
    return monkeyctr[0] * monkeyctr[1];
}

fn part2(monkeys: Vec<Monkey>, monkeyfns: Vec<Box<dyn Fn(i64) -> i64>>, mut monkeyitems: Vec<LinkedList<i64>>) -> i64 {

    let monkeyfactor = monkeys.iter().fold(1, |a, b| a * b.test);

    let mut monkeyctr = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for (mindex, monkey) in monkeys.iter().enumerate() {
            for item in monkeyitems[mindex].clone() {
                monkeyctr[mindex] += 1;
                let worry = (monkeyfns[mindex])(item) % monkeyfactor;
                if worry % monkey.test == 0 {
                    monkeyitems[monkey.truetest].push_back(worry);
                    println!("Monkey {} gives {} to {}", mindex, worry, monkey.truetest);
                } else {
                    monkeyitems[monkey.falsetest].push_back(worry);
                    println!("Monkey {} gives {} to {}", mindex, worry, monkey.falsetest);
                }
            }
            monkeyitems[mindex] = LinkedList::new();
        }
        for idx in 0..monkeyctr.len() {
            println!("{}: {} ", idx, monkeyctr[idx]);
        }
    }
    monkeyctr.sort_by(|a, b| b.cmp(a));
    return monkeyctr[0] * monkeyctr[1];
}

fn main() {
    let lines = io::stdin().lines();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkeyitems: Vec<LinkedList<i64>> = Vec::new();
    let mut monkeyfns = Vec::new();
 
    let monkeylines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    for monkey in monkeylines.chunks(7) {
        let (_, itemstr) = monkey[1].split_at(18);
        let items: LinkedList<i64> = itemstr.split(", ").map(|item| item.parse::<i64>().unwrap()).collect();
        let (_, test) = monkey[3].split_at(21);
        let (_, testtrue) = monkey[4].split_at(29);
        let (_, testfalse) = monkey[5].split_at(30);
        let tokens: Vec<&str> = monkey[2].split(" ").collect();
        monkeys.push(Monkey { 
            test: test.parse::<i64>().unwrap(), 
            truetest: testtrue.parse::<i64>().unwrap() as usize,
            falsetest: testfalse.parse::<i64>().unwrap() as usize,
        });
        monkeyitems.push(items.clone());
        monkeyfns.push(operation(tokens[5], tokens[6], tokens[7]));
    }

    // println!("part1 {}", part1(monkeys, monkeyfns, monkeyitems, 3, 20));
    println!("part2 {}", part2(monkeys, monkeyfns, monkeyitems));

 

}

