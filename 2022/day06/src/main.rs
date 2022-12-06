use std::{io, collections::{VecDeque, HashSet}};

fn find_marker(line: String, length: usize) -> Option<i32> {
    let mut queue: VecDeque<char> = VecDeque::new();
    let mut index = 1;
    for ch in line.chars() {
        queue.push_back(ch);
        if queue.len() > length {
            queue.pop_front();
            let set: HashSet<char> = HashSet::from_iter(queue.clone());
            if set.len() == length {
                return Some(index);
            }
        }
        index += 1;
    }
    return None;
}

fn main() {
    let lines = io::stdin().lines();

    let mut part1 = None;
    let mut part2 = None;

    for line in lines {
        let whole = line.unwrap();

        part1 = find_marker(whole.clone(), 4);
        part2 = find_marker(whole.clone(), 14);
    }

    println!("part1: {}", part1.unwrap());
    println!("part2: {}", part2.unwrap());

}