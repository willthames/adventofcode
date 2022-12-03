use std::io;

fn main() -> io::Result<()> {
    let lines = io::stdin().lines();
    let mut elves = Vec::new();
    let mut index = 0;
    elves.push(0);
    for line in lines {
        let data = line.unwrap();
        if data == "" {
            index += 1;
            elves.push(0);
        } else {
            elves[index] += data.parse::<i32>().unwrap();
        }
    }
    let max_value = elves.iter().max();
    match max_value {
        None => println!("No elves!"),
        Some(max) => println!("{}", max)
    }
    Ok(())
}
