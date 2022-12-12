use std::io;

use pathfinding::prelude::bfs;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i16, pub i16);

pub struct Board {
    pub width: u8,
    pub height: u8,
    pub data: Vec<Vec<char>>,
    pub allow_diagonal: bool,
    pub start: Option<Pos>,
    pub end: Option<Pos>,
}

impl Board {
    pub fn new(board_lines: Vec<String>, allow_diagonal: bool) -> Board {
        let width = board_lines[0].len() as u8;
        let height = board_lines.len() as u8;
        let mut data = Vec::new();
        let mut start: Option<Pos> = None;
        let mut end: Option<Pos> = None;
        for (ridx, board_line) in board_lines.iter().enumerate() {
            let mut row: Vec<char> = Vec::new();
            for (cidx, c) in board_line.chars().enumerate() {
                match c {
                    'S' => {
                        row.push('a');
                        start = Some(Pos(cidx as i16, ridx as i16));
                    },
                    'E' => {
                        row.push('z');
                        end = Some(Pos(cidx as i16, ridx as i16));
                    },
                    _ => row.push(c),
                }
            }
            data.push(row);
        }
        Board {width, height, data, allow_diagonal, start, end}
    }

    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        let curr_value = self.data[position.1 as usize][position.0 as usize];
        for dx in -1i16..=1 {
            for dy in -1i16..=1 {
                if self.allow_diagonal {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                }
                else {
                    // Omit diagonal moves (and moving to the same position)
                    if (dx + dy).abs() != 1 {
                        continue;
                    }
                }
                let new_position = Pos(position.0 + dx, position.1 + dy);
                if new_position.0 < 0 || new_position.0 >= self.width.into() || new_position.1 < 0 || new_position.1 >= self.height.into() {
                    continue;
                }
                let board_value = self.data[new_position.1 as usize][new_position.0 as usize];
                if (board_value as i8) - (curr_value as i8) <= 1 {
                    successors.push(Successor { pos: new_position, cost: 1});
                }
            }
        }

        successors
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u8,
}

fn path(board: &Board, start: Pos, end: Pos) -> Option<Vec<Pos>> {
    bfs(
        &start,
        |p| board.get_successors(p).iter().map(|successor| successor.pos).collect::<Vec<_>>(), 
        |p| *p==end)
}

fn main() {

    let lines = io::stdin().lines();
    let mut boardlines: Vec<String> = Vec::new();
    for line in lines {
        let whole = line.unwrap();
        boardlines.push(whole);
    }

    let board = Board::new(boardlines, false);

    let part1 = path(&board, 
        board.start.expect("No start found"),
        board.end.expect("No end found")
    );
    println!("part1: {}", part1.unwrap().len() - 1);

    let mut lengths = Vec::new();
    for (ridx, row) in board.data.iter().enumerate() {
        for (cidx, ch) in row.iter().enumerate() {
            if *ch == 'a' {
                let this = path(&board, Pos(cidx as i16, ridx as i16), board.end.unwrap());
                if this.is_some() {
                    lengths.push(this.unwrap().len() - 1);
                }
            }
        }
    }

    println!("part2: {}", lengths.iter().min().unwrap());
}