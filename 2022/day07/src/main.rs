use std::{io, collections::HashMap};

fn part1(tree: HashMap<String, Option<i32>>) -> i32 {
    let mut total = 0;
    for element in tree.iter().filter(|(_k, v)| v.is_none()).map(|(k, _v)| k) {
        let size: i32 = tree.iter().filter(|(k, v)| k.starts_with(element) && v.is_some()).map(|(_k,v)| v.unwrap()).sum();
        if size < 100000 {
            total += size;
        }
    }
    return total
}

fn part2(tree: HashMap<String, Option<i32>>) -> i32 {
    let mut sizes: HashMap<String, i32> = HashMap::new();
    for element in tree.iter().filter(|(_k, v)| v.is_none()).map(|(k, _v)| k) {
        let size: i32 = tree.iter().filter(|(k, v)| k.starts_with(element) && v.is_some()).map(|(_k,v)| v.unwrap()).sum();
        sizes.insert(element.to_string(), size);
    }
    let needed = sizes["//"] - 40000000;
    return sizes.iter().filter(|(_k, v)| **v > needed).map(|(_k, v)| *v).min().unwrap()
}


fn main() {
    let lines = io::stdin().lines();

    let mut path: Vec<String> = Vec::new();
    let mut tree: HashMap<String, Option<i32>> = HashMap::new();
    tree.insert("//".to_string(), None);
    for line in lines {
        let whole = line.unwrap();
        if whole.starts_with("$ cd") {
            let (_, dir) = whole.split_at(5);
            if dir == ".." {
                path.pop();
            } else {
                path.push(dir.to_string());
            }
        } else {
            if whole.starts_with("$ ls") {
                continue;
            } else {
                let tokens = whole.splitn(2, " ").collect::<Vec<&str>>();
                let fullpath = format!("{}/{}", path.join("/"), tokens[1]);
                if whole.starts_with("dir") {
                    tree.insert(fullpath, None);
                }
                else {
                    tree.insert(fullpath, Some(tokens[0].parse::<i32>().unwrap()));
                }
            }
        }
    }


    println!("{}", part1(tree.clone()));
    println!("{}", part2(tree));
        
}
