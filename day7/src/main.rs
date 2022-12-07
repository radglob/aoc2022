use std::fs;

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    children: Vec<Directory<'a>>,
    files: Vec<File>,
    parent: Box<Option<&'a Directory<'a>>>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

fn handle_command(command_string: &str) {
}

fn parse_directory(input: &str) -> Directory {
    let mut lines = input.lines();
    lines.next();
    let mut current_directory: Directory = Directory { name: "/".to_string(), children: vec![], files: vec![], parent: Box::new(None) };

    while let Some(line) = lines.next() {
        let mut cs = line.chars();
        if let Some('$') = cs.next() {
            cs.next();
            let command_string: String = cs.collect();
            handle_command(&command_string);
            println!("{}", command_string);
        } else {
            // Either a file or directory.
            println!("{}", line);
        }
    }

    current_directory
}

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("src/basic_input.txt")?;
    let directory: Directory = parse_directory(&contents);
    println!("{:?}", directory);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
