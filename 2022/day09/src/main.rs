#![allow(dead_code)]

use std::{io, collections::HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow() {
        let head = Position{left: 5, up: 2};
        let tail = Position{left: 3, up: 1};
        assert_eq!(follow(tail, head), Position{left: 4, up: 2});
    }
    #[test]
    fn test_nearer() {
        assert_eq!(nearer(1, 2), 2);
        assert_eq!(nearer(-1, -2), -2);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Position {
    left: i32,
    up: i32,
}

#[derive(Clone)]
struct Instruction {
    distance: i32,
    direction: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction(dir: &str) -> Option<Direction> {
    match dir {
        "R " => Some(Direction::Right),
        "L " => Some(Direction::Left),
        "U " => Some(Direction::Up),
        "D " => Some(Direction::Down),
        _ => None,
    }
}

fn nearer(tail: i32, head: i32) -> i32{
    if tail > head {
        tail - 1
    } else {
        if tail < head {
             tail + 1
        } else {
            tail
        }
    }
}



fn follow(tail: Position, head: Position) -> Position {
    if tail.up - head.up > 1 {
        return Position{left: nearer(tail.left, head.left), up: tail.up - 1};
    }
    if tail.left - head.left > 1 {
        return Position{left: tail.left - 1, up: nearer(tail.up, head.up)};
    }
    if head.up - tail.up > 1 {
        return Position{left: nearer(tail.left, head.left), up: tail.up + 1};
    }
    if head.left - tail.left > 1 {
        return Position{left: tail.left + 1, up: nearer(tail.up, head.up)};
    }
    return Position{left: tail.left, up: tail.up};
}

fn update(head: Position, direction: Direction) -> Position {
    match direction {
        Direction::Right => Position{left: head.left + 1, up: head.up},
        Direction::Up => Position{left: head.left, up: head.up + 1},
        Direction::Left => Position{left: head.left - 1, up: head.up},
        Direction::Down => Position{left: head.left , up: head.up - 1},
    }
}

fn part1(instructions: Vec<Instruction>) -> usize {
    let mut head = Position{left: 0, up: 0};
    let mut tail = Position{left: 0, up: 0};
    let mut visited: Vec<Position> = Vec::new();
    for instruction in instructions {
        for _ in 0..instruction.distance {
            head = update(head, instruction.direction);
            tail = follow(tail, head);
            visited.push(Position{left: tail.left, up: tail.up});
        }
    }
    let unique: HashSet<Position> = HashSet::from_iter(visited);
    return unique.len();
}

fn show(elements: Vec<Position>) {
    let xmax = elements.iter().map(|e| e.left).max().unwrap();
    let xmin = elements.iter().map(|e| e.left).min().unwrap();
    let ymax = elements.iter().map(|e| e.up).max().unwrap();
    let ymin = elements.iter().map(|e| e.up).min().unwrap();

    for y in (ymin..ymax+1).rev() {
        for x in xmin..xmax+1 {
            if elements.contains(&Position{left:x, up:y}) {
                let idx = elements.iter().position(|p| p == &Position{left:x, up:y}).unwrap();
                print!("{}", idx);
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
    println!("");
}

fn part2(instructions: Vec<Instruction>) -> usize {
    let mut rope: Vec<Position> = vec![Position{left: 0, up: 0}; 10];
    let mut visited: Vec<Position> = Vec::new();
    for instruction in instructions {
        for _ in 0..instruction.distance {
            rope[0] = update(rope[0], instruction.direction);
            for knot in 1..10 {
                rope[knot] = follow(rope[knot], rope[knot-1]);
            }
            visited.push(Position{left: rope[9].left, up: rope[9].up});
            // show(rope.clone());
        }
    }
    let unique: HashSet<Position> = HashSet::from_iter(visited);
    return unique.len();
}

fn main() {
   let lines = io::stdin().lines();
   let mut instructions: Vec<Instruction> = Vec::new();

   for line in lines {
        let whole = line.unwrap();
        let (dir, diststr) = whole.split_at(2);
        let distance = diststr.parse::<i32>().unwrap();
        instructions.push(Instruction{distance: distance, direction: direction(dir).unwrap()});
   }

    println!("part1 {}",part1(instructions.clone()));
    println!("part2 {}",part2(instructions));

}