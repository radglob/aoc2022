use std::fs;
use std::str::Split;
use std::rc::Rc;

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Directory>,
    files: Vec<File>,
    parent: Option<Rc<Directory>>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(String),
    List
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(String, usize)
}

fn parse_command(mut tokens: Split<&str>) -> Command {
    match tokens.next() {
        Some("cd") => {
            let directory_name = tokens.next().unwrap().to_string();
            Command::ChangeDirectory(directory_name)
        },
        Some("ls") => Command::List,
        _ => unreachable!()
    }
}

fn parse_directory(input: &str) -> Directory {
    let mut lines = input.lines();
    lines.next();
    let current_directory: Directory = Directory { name: "/".to_string(), children: vec![], files: vec![], parent: None };

    while let Some(line) = lines.next() {
        let mut tokens = line.split(" ");
        match tokens.next() {
            Some("$") => {
                let command = parse_command(tokens);
                println!("{:?}", command);
            },
            Some("dir") => {
                let dir_name = tokens.next().unwrap().to_string();
                let dir = Entry::Dir(dir_name);
                println!("{:?}", dir);
            },
            Some(number) => {
                let file_size = number.parse::<usize>().unwrap();
                let filename = tokens.next().unwrap().to_string();
                let file = Entry::File(filename, file_size);
                println!("{:?}", file);
            },
            _ => unreachable!()
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
