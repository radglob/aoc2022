use std::fs;
use std::collections::HashSet;

fn priority(c: char) -> u32 {
    if ('A'..='Z').contains(&c) {
        return (c as u32) - ('A' as u32) + 27
    }

    (c as u32) - ('a' as u32) + 1
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Something went wrong reading the file");
    let mut sum = 0;
    /* Part 1
    for line in contents.lines() {
        let pivot = line.len() / 2;
        let (first_compartment, second_compartment) = line.split_at(pivot);
        let s1: HashSet<char> = first_compartment.chars().collect();
        let s2: HashSet<char> = second_compartment.chars().collect();
        let common = s1.intersection(&s2).next();
        let priority: u32 = match common {
            Some(c) => priority(*c),
            None => 0
        };
        sum += priority;
    */

    /* Part 2 */
    let lines: Vec<&str> = contents.lines().collect();
    for group in lines.chunks(3) {
        let mut sets = group.into_iter().map(|l| {
            let s: HashSet<char> = l.chars().collect();
            s
        });
        let mut s = sets.next().unwrap();
        for set in sets {
            s = s.intersection(&set).copied().collect();
        }
        let priority = match s.into_iter().next() {
            Some(c) => priority(c),
            None => 0
        };
        sum += priority; 
    }
    println!("Sum of priorities: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('Z'), 52);
        assert_eq!(priority('z'), 26);
    }
}
