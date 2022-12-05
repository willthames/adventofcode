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
    elves.sort_by(|a, b| b.cmp(a));

    if elves.len() < 3 {
        println!("Not enough elves!");
    } else {
        println!("{}", elves[0] + elves[1] + elves[2]);
    }
    Ok(())
}
