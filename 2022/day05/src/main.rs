use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crates_reader() {
        let threecrates = cratesreader("[A] [B]     [C]".to_string());
        assert_eq!(threecrates.len(), 4);
        assert_eq!(threecrates[0], 'A');
        assert_eq!(threecrates[2], ' ');
    }
    #[test]
    fn test_crates_reader_more_empty() {
        let onecrate = cratesreader("    [D]    ".to_string());
        assert_eq!(onecrate.len(), 3);
        assert_eq!(onecrate[1], 'D');
    }
    #[test]
    fn test_instructions_reader() {
        let instructions = instructionsreader("move 11 from 7 to 9".to_string());
        assert_eq!(instructions.count, 11);
        assert_eq!(instructions.from, 6);
        assert_eq!(instructions.to, 8);

    }
}
#[derive(Copy, Clone)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn cratesreader(line: String) -> Vec<char> {
    return line.chars().collect::<Vec<char>>().chunks(4).map(|slice| slice[1]).collect();
}

fn instructionsreader(line: String) -> Instruction {
    let tokens: Vec<&str> = line.splitn(6, " ").collect();
    return Instruction {
         count: tokens[1].parse::<usize>().unwrap(), 
         from: tokens[3].parse::<usize>().unwrap() - 1, 
         to: tokens[5].parse::<usize>().unwrap() - 1,
    }
}

fn part1(mut crates: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let from = crates[instruction.from].pop().unwrap();
            crates[instruction.to].push(from);
        }
    }
    return crates.iter().map(|crat| *crat.last().unwrap()).collect::<String>()
}

fn part2(mut crates: Vec<Vec<char>>, instructions: Vec<Instruction>) -> String {
    for instruction in instructions {
        let split = crates[instruction.from].len() - instruction.count;
        let from = crates[instruction.from].split_off(split);            
        crates[instruction.to].extend(from);
    }
    return crates.iter().map(|crat| *crat.last().unwrap()).collect::<String>()
}

fn main() {
    let lines = io::stdin().lines();

    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut stage2 = false;
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        let whole = line.unwrap();
        if whole == "" {
            stage2 = true;
            continue;
        }
        if stage2 {
            instructions.push(instructionsreader(whole));
        } else {
            // ignore numbering line
            if !whole.contains("[") {
                continue;
            }
            let thesecrates = cratesreader(whole);
            let mut index = 0;
            for thiscrate in thesecrates {
                if index >= crates.len() {
                    crates.push(Vec::new());
                }
                if thiscrate != ' ' {
                    crates[index].splice(0..0, vec![thiscrate]);
                }
                index += 1;
            }
        }
    }
    

    println!("part1: {}", part1(crates.clone(), instructions.clone()));
    println!("part2: {}", part2(crates.clone(), instructions.clone()));

}
