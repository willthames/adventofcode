use std::io;

fn part1(values: Vec<i32>) -> i32 {
    let mut total = 0;
    for cycle in (0..values.len()).skip(19).step_by(40).collect::<Vec<usize>>() {
        total += values[cycle] * (cycle as i32 + 1); 
    }
    return total
}

fn part2(values: Vec<i32>) {
    let mut cycle = 0;
    for value in values {
        if cycle >= value - 1 && cycle <= value + 1 {
            print!("#");
        } else {
            print!(".");
        }
        cycle += 1;
        if cycle == 40 {
            println!("");
            cycle -= 40;
        }
    }
}


fn main() {
    let lines = io::stdin().lines();

    let mut x = 1;
    let mut values: Vec<i32> = Vec::new();
    values.push(1);

    for line in lines {
        let whole = line.unwrap();
        if whole == "noop" {
            values.push(x);
        }
        else {
            let tokens: Vec<&str> = whole.splitn(2, " ").collect();
            let value = tokens[1].parse::<i32>().unwrap();
            values.push(x);
            x += value;
            values.push(x);
        }

    }

    println!("part1 {}",part1(values.clone()));
    part2(values);
}