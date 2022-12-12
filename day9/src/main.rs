use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct MoveInstruction {
    direction: Direction,
    steps: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn difference(&self, p: Point) -> Point {
        Point::new(self.x - p.x, self.y - p.y)
    }

    fn is_adjacent(&self, p: Point) -> bool {
        let diff = self.difference(p);
        if diff.x.abs() >= 2 || diff.y.abs() >= 2 {
            return false
        } else {
            return true
        }
    }
}

struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert(Point::new(0, 0));

        Self {
            knots: vec![Point::new(0, 0); size],
            visited
        }
    }

    fn update(&mut self, motion: MoveInstruction) {
        let increment: Point = match motion.direction {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        };
        for _ in 0..motion.steps {
            let mut head = &mut self.knots[0];
            head.x += increment.x;
            head.y += increment.y;
            for i in 1..self.knots.len() {
                if !self.knots[i].is_adjacent(self.knots[i-1]) {
                    let mut delta = self.knots[i-1].difference(self.knots[i]);
                    delta.x = delta.x.clamp(-1, 1);
                    self.knots[i].x += delta.x;
                    delta.y = delta.y.clamp(-1, 1);
                    self.knots[i].y += delta.y;
                }
            }
            self.visited.insert(*self.knots.last().unwrap());
        }
    }
}

fn parse_move_instructions(contents: &str) -> Vec<MoveInstruction> {
    contents
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            match iter.next().unwrap() {
                "U" => MoveInstruction {
                    direction: Direction::Up,
                    steps: iter.next().unwrap().parse().unwrap(),
                },
                "D" => MoveInstruction {
                    direction: Direction::Down,
                    steps: iter.next().unwrap().parse().unwrap(),
                },
                "R" => MoveInstruction {
                    direction: Direction::Right,
                    steps: iter.next().unwrap().parse().unwrap(),
                },
                "L" => MoveInstruction {
                    direction: Direction::Left,
                    steps: iter.next().unwrap().parse().unwrap(),
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    let contents = include_str!("input.txt");
    let motions = parse_move_instructions(contents);
    let mut rope = Rope::new(10);
    for motion in motions {
        rope.update(motion);
    }
    println!("Unique tail positions: {}", rope.visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_adjacency() {
        let p = Point::new(0, 0);
        assert!(p.is_adjacent(Point::new(0, 0)));
        assert!(p.is_adjacent(Point::new(1, 0)));
        assert!(p.is_adjacent(Point::new(1, 1)));
        assert!(!p.is_adjacent(Point::new(2, 0)));
    }
}
