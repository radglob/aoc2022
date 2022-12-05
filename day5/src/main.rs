use std::fs;
use std::str;

fn parse_crate_setup(config_str: &str) -> Vec<Vec<&str>> {
    let crate_config: Vec<Vec<&str>> = config_str.lines().map(|line| {
        let parts: Vec<&str> = line.as_bytes().chunks(4).map(|c| { 
            str::from_utf8(c).expect("Unable to convert bytes to str").trim()
        }).collect();
        parts
    }).collect();
    let mut rotated_config: Vec<Vec<&str>> = vec![vec![""; crate_config.len()]; crate_config[0].len()];
    for i in 0..crate_config[0].len() {
        for j in 0..crate_config.len() {
            rotated_config[i][j] = crate_config[j][i];
        }
        rotated_config[i].reverse();
    }
    rotated_config
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("./src/basic_input.txt")?;
    let instructions: Vec<&str> = contents.split("\n\n").collect();
    let crate_setup = parse_crate_setup(instructions[0]);
    println!("{:?}", crate_setup);
    Ok(())
}
