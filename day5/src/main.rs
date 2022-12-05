use std::fs;
use std::str;

#[derive(Debug)]
struct MoveInstruction {
    source: usize,
    target: usize,
    count: usize
}

fn parse_crate_value(crate_str: String) -> String {
    let mut input = crate_str.chars();
    while let Some(c) = input.next() {
        if c != '[' && c != ']' {
            let slice = &[c as u8];
            let v = str::from_utf8(slice).expect("There was an issue converting from utf8");
            return v.to_string();
        }
    }
    return "".to_string();
}

fn parse_move_instruction(instruction: &str) -> MoveInstruction {
    let mut tokens = instruction.split_whitespace();
    let mut source = 0;
    let mut target = 0;
    let mut count = 0;
    while let Some(token) = tokens.next() {
        if token == "move" {
            count = tokens.next().unwrap().parse::<usize>().unwrap();
        } else if token == "from" {
            source = tokens.next().unwrap().parse::<usize>().unwrap();
        } else if token == "to" {
            target = tokens.next().unwrap().parse::<usize>().unwrap();
        }
    }
    MoveInstruction { count, source, target }
}

fn parse_stacks(config_str: &str) -> Vec<Vec<String>> {
    let crate_config: Vec<Vec<String>> = config_str.lines().map(|line| {
        let parts: Vec<String> = line.as_bytes().chunks(4).map(|c| { 
            str::from_utf8(c).expect("Unable to convert bytes to str").trim().to_string()
        }).collect();
        parts
    }).collect();
    let mut rotated_config: Vec<Vec<String>> = vec![vec!["".to_string(); crate_config.len()]; crate_config[0].len()];
    for i in 0..crate_config[0].len() {
        for j in 0..crate_config.len() {
            rotated_config[i][j] = crate_config[j][i].clone()
        }
        rotated_config[i].reverse();
        rotated_config[i].remove(0);
        rotated_config[i] = rotated_config[i].clone().into_iter().filter(|s| s != "").map(parse_crate_value).collect();
    }
    rotated_config
}

fn individual_move(stacks: &mut Vec<Vec<String>>, move_instruction: MoveInstruction) {
    for _ in 0..move_instruction.count {
        let c = stacks[move_instruction.source - 1].pop().unwrap();
        stacks[move_instruction.target - 1].push(c);
    }
}

fn group_move(stacks: &mut Vec<Vec<String>>, move_instruction: MoveInstruction) {
    let stack = &stacks[move_instruction.source - 1];
    let index = stack.len() - move_instruction.count;
    let mut crates = stacks[move_instruction.source - 1].split_off(index);
    stacks[move_instruction.target - 1].append(&mut crates);
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let instructions: Vec<&str> = contents.split("\n\n").collect();
    let mut stacks = parse_stacks(instructions[0]);
    for line in instructions[1].lines() {
        let move_instruction = parse_move_instruction(line);
        // individual_move(&mut stacks, move_instruction);
        group_move(&mut stacks, move_instruction);
    }
    for mut stack in stacks {
        let top_crate = stack.pop().unwrap();
        print!("{}", top_crate);
    }
    Ok(())
}
