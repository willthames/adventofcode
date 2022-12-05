use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_works() {
        assert_eq!(contains(vec!["23", "29"], vec!["24", "28"]), true);
        assert_eq!(contains(vec!["23", "27"], vec!["24", "28"]), false);
        assert_eq!(contains(vec!["23", "28"], vec!["24", "28"]), true);

    }

}


fn contains(left: Vec<&str>, right: Vec<&str>) -> bool {
    (str::parse::<u32>(left[0]).unwrap() <= str::parse::<u32>(right[0]).unwrap() &&
    str::parse::<u32>(left[1]).unwrap() >= str::parse::<u32>(right[1]).unwrap()) || 
    (str::parse::<u32>(left[0]).unwrap() >= str::parse::<u32>(right[0]).unwrap() &&
    str::parse::<u32>(left[1]).unwrap() <= str::parse::<u32>(right[1]).unwrap())
}

fn main() {

    let lines = io::stdin().lines();
    let mut total: u32 = 0;

    for line in lines {
        let whole = line.unwrap();
        let pairs: Vec<&str> = whole.splitn(2, ',').collect();
        let left = pairs[0].splitn(2, '-').collect();
        let right = pairs[1].splitn(2, '-').collect();
        if contains(left, right) {
            total += 1;
        }
    }

    println!("{}", total);
}