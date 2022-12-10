use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("src/input.txt")?;
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in contents.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    };

    // let sizes: u32 = sizes.into_values().filter(|size| *size <= 100_000).sum();
    // println!("{}", sizes);
    let disk = 70_000_000;
    let needed = 30_000_000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = disk - root;

    let sizes = sizes.into_values().filter(|size| available + size >= needed).min().unwrap();
    println!("{}", sizes);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
