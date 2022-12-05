use std::collections::HashSet;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_case_works() {
        assert_eq!(score('C'), 29);
    }
    #[test]
    fn lower_case_works() {
        assert_eq!(score('c'), 3);
    }
}

fn score(ch: char) -> u32 {
    if ch >= 'A' && ch <='Z' {
        return (ch as u32) - 65 + 27;
    } else {
        return (ch as u32) - 97 + 1;
    }
}

fn main() {

    let lines = io::stdin().lines();
    let mut total: u32 = 0;
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut index: usize = 0;
    groups.resize(3, HashSet::from(['A']));

    for line in lines {
        let whole = line.unwrap();
        groups[index] = HashSet::from_iter(whole.chars());
        index = (index + 1) % 3;
        if index == 0 {
            let firstsecond: HashSet<char> = groups[0].intersection(&groups[1]).cloned().collect();
            let badge= firstsecond.intersection(&groups[2]).next().unwrap();
            total += score(*badge);
        }
    }

    println!("{}", total);
}