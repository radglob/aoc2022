use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/actual_input.txt")
        .expect("Something went wrong reading the file");
    let answer = part2(&contents);
    println!("The answer is {}", answer)
}

fn part1(input: &str) -> u32 {
    let line_groups = input.split("\n\n");
    let max_calories = line_groups
        .map(|group| {
            group
                .split("\n")
                .filter(|&s| !s.is_empty())
                .map(|line| {
                    line.parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .max()
        .unwrap();
    return max_calories;
}

fn part2(input: &str) -> u32 {
    let line_groups = input.split("\n\n");
    let mut calories: Vec<u32> = line_groups
        .map(|group| {
            group
                .split("\n")
                .filter(|&s| !s.is_empty())
                .map(|line| {
                    line.parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    calories.sort();
    calories.reverse();
    let top3 = &calories[0..3];
    top3.into_iter().sum()
}
