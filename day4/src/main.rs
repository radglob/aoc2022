use std::fs;

fn contains(p0: (u32, u32), p1: (u32, u32)) -> bool {
    p0.0 <= p1.0 && p0.1 >= p1.1
}

fn overlaps(p0: (u32, u32), p1: (u32, u32)) -> bool {
    (p0.0 <= p1.0 && p0.1 >= p1.0) || (p1.0 <= p0.0 && p1.1 >= p0.0)
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let lines = contents.lines();
    let mut count = 0;
    for line in lines {
        let pairs: Vec<(u32, u32)> = line.split(",").map(|s| {
            let bounds: Vec<u32> = s.split("-").map(|b| b.parse::<u32>().expect("Failed to parse number")).collect();
            (bounds[0], bounds[1])
        }).collect();
        // if contains(pairs[0], pairs[1]) || contains(pairs[1], pairs[0]) {
        //     count += 1
        // }
        if overlaps(pairs[0], pairs[1]) {
            count += 1
        }
    }
    println!("{}", count);
    Ok(())
}
