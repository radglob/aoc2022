use std::collections::HashSet;
use std::fs;

struct Buffer<'a> {
    start: usize,
    end: usize,
    length: usize,
    input: &'a str,
}

impl Buffer<'_> {
    fn all_different(&self) -> bool {
        let set: HashSet<char> = self.input[self.start..=self.end].chars().collect();
        set.len() == self.length
    }

    fn shift(&mut self) {
        if self.end - self.start < self.length - 1 {
            self.end += 1;
        } else {
            self.start += 1;
            self.end += 1;
        }
    }
}

fn start_of_packet(data: &str) -> usize {
    let mut buffer = Buffer {
        start: 0,
        end: 0,
        input: data,
        length: 4,
    };
    while let false = buffer.all_different() {
        buffer.shift();
    }
    buffer.end + 1
}

fn start_of_message(data: &str) -> usize {
    let mut buffer = Buffer {
        start: 0,
        end: 0,
        input: data,
        length: 14,
    };
    while let false = buffer.all_different() {
        buffer.shift();
    }
    buffer.end + 1
}

fn main() -> Result<(), std::io::Error> {
    let contents = fs::read_to_string("./src/input.txt")?;
    let start_index = start_of_message(&contents);
    println!("{}", start_index);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_5() {
        assert_eq!(start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn should_return_6() {
        assert_eq!(start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn should_return_7() {
        assert_eq!(start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn should_return_10() {
        assert_eq!(start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn should_return_11() {
        assert_eq!(start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn should_return_19() {
        assert_eq!(start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn should_return_23() {
        assert_eq!(start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn should_return_26() {
        assert_eq!(start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn should_return_29() {
        assert_eq!(start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }
}
