type Register = i32;

struct CPU {
    x: Register,
    cycle_count: i32,
    signal_strengths: Vec<i32>,
    screen_buffer: [[char; 40]; 6],
}

enum Command {
    AddX(i32),
    Noop,
}

impl CPU {
    fn new() -> Self {
        Self {
            x: 1,
            cycle_count: 0,
            signal_strengths: vec![],
            screen_buffer: [['.'; 40]; 6],
        }
    }

    fn draw_pixel(&mut self) {
        let x = (self.cycle_count - 1) % 40;
        let y = self.cycle_count / 40;
        if self.x >= (x-1) as i32 && self.x <= (x+1) as i32 {
            self.screen_buffer[y as usize][x as usize] = '#'
        }
    }

    fn increment_cycle_count(&mut self) {
        self.cycle_count += 1;
        if (self.cycle_count - 20) % 40 == 0 {
            let signal_strength = self.cycle_count as i32 * self.x;
            self.signal_strengths.push(signal_strength);
        }
        self.draw_pixel();
        for line in self.screen_buffer {
            let s: String = line.iter().collect();
            println!("{}", s);
        }
        println!("");
    }

    fn execute(&mut self, command: Command) {
        match command {
            Command::AddX(n) => {
                self.increment_cycle_count();
                self.increment_cycle_count();
                self.x += n;
            }
            Command::Noop => {
                self.increment_cycle_count();
            }
        }
    }
}

fn parse_commands(contents: &str) -> Vec<Command> {
    contents
        .lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("addx") => {
                    let i: i32 = tokens.next().unwrap().parse().unwrap();
                    Command::AddX(i)
                }
                Some("noop") => Command::Noop,
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    let contents = include_str!("input.txt");
    let commands = parse_commands(contents);
    let mut cpu = CPU::new();
    for command in commands {
        cpu.execute(command);
    }
    let _total_signal_strengths: i32 = cpu.signal_strengths.iter().sum();
}
