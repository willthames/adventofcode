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

    for line in lines {
        let whole = line.unwrap();
        let (first, second) = whole.split_at(whole.len()/2);
        let firstchars: HashSet<char> = HashSet::from_iter(first.chars());
        let secondchars: HashSet<char> = HashSet::from_iter(second.chars());
        let error = firstchars.intersection(&secondchars).next().unwrap();
        total += score(*error);
    }

    println!("{}", total);
}