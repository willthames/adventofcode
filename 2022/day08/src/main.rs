use std::io;

fn higher(tree: u32, trees: &[u32]) -> bool {
    return trees.iter().all(|t| *t < tree);
}

fn visible(ridx: usize, cidx: usize, row: &[u32], col: &[u32]) -> bool {
    let tree = row[cidx];
    return
        higher(tree, &row[..cidx]) ||
        higher(tree, &row[cidx+1..]) ||
        higher(tree, &col[..ridx]) ||
        higher(tree, &col[ridx+1..])
}

fn part1(rows: Vec<Vec<u32>>, cols: Vec<Vec<u32>>) -> usize {
    let mut total = 2 * rows.len() + 2 * cols.len() - 4;
    for row in 1..rows.len() - 1 {
        for col in 1..cols.len() - 1 {
            if visible(row, col, rows[row].as_slice(), cols[col].as_slice()) {
                total += 1;
            }
        }
    }
    return total
}

fn scenic(tree: u32, trees: &[u32], reverse: bool) -> usize {
    let mut result : usize = 0;
    if reverse {
        for i in (0..trees.len()).rev() {
            if trees[i] >= tree {
                return result + 1;
            } else {
                result += 1;
            }
        }
    } else {
        for i in 0..trees.len() {
            if trees[i] >= tree {
                return result + 1;
            } else {
                result += 1;
            }
        }    
    }

    return result
}

fn scenic_total(ridx: usize, cidx: usize, row: &[u32], col: &[u32]) -> usize {
    let tree = row[cidx];
    return 
        scenic(tree, &row[..cidx], true) *
        scenic(tree, &row[cidx+1..], false) *
        scenic(tree, &col[..ridx], true) *
        scenic(tree, &col[ridx+1..], false)
}

fn part2(rows: Vec<Vec<u32>>, cols: Vec<Vec<u32>>) -> usize {
    let mut max = 0;
    for row in 1..rows.len() - 1 {
        for col in 1..cols.len() - 1 {
            let score = scenic_total(row, col, rows[row].as_slice(), cols[col].as_slice());
            if score > max {
                max = score;
            }
        }
    }
    return max
}


fn main() {
    let lines = io::stdin().lines();

    let mut rows: Vec<Vec<u32>> = Vec::new();
    let mut cols: Vec<Vec<u32>> = Vec::new();
    let mut first = true;
    for (row, line) in lines.enumerate() {
        rows.push(Vec::new());
        let whole = line.unwrap();
        let trees = whole.chars();
        for (col, tree) in trees.enumerate() {
            if first {
                cols.push(Vec::new());
            }
            rows[row].push(tree.to_digit(10).unwrap());
            cols[col].push(tree.to_digit(10).unwrap());
        }
        first = false;
    }

    println!("part1: {}", part1(rows.clone(), cols.clone()));
    println!("part2: {}", part2(rows, cols));

}
