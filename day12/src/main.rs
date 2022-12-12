use std::collections::HashSet;

type Cells = Vec<Vec<char>>;

fn get_position(cells: &Cells, c: &char) -> Option<(i32, i32)> {
    for (i, row) in cells.iter().enumerate() {
        if row.contains(c) {
            let col = row.iter().position(|e| e == c).unwrap();
            return Some((i.try_into().unwrap(), col.try_into().unwrap()));
        }
    }
    None
}


struct HeightMap {
    cells: Vec<Vec<char>>,
    current_pos: (i32, i32),
    previous_positions: HashSet<(i32, i32)>,
    end_pos: (i32, i32),
    move_count: u32
}

impl HeightMap {
    fn parse(contents: &str) -> Self {
        let cells: Vec<Vec<char>> = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let current_pos = get_position(&cells, &'S').unwrap();
        let end_pos = get_position(&cells, &'E').unwrap();
        let mut previous_positions = HashSet::new();
        previous_positions.insert(current_pos);
        Self { 
            cells,
            current_pos,
            end_pos,
            move_count: 0,
            previous_positions
        }
    }

    fn finished(&self) -> bool {
        self.current_pos == self.end_pos
    }

    fn possible_moves(&self) -> Vec<(i32, i32)> {
        let mut moves: Vec<(i32, i32)> = vec![];
        let indices = [
            (self.current_pos.0 + 1, self.current_pos.1),
            (self.current_pos.0 - 1, self.current_pos.1),
            (self.current_pos.0, self.current_pos.1 - 1),
            (self.current_pos.0, self.current_pos.1 + 1),
        ];
        for (x, y) in indices {
            if x < 0 || x > self.cells[0].len() as i32 || y < 0 || y > self.cells.len() as i32 {
                ()
            } else {
                // TODO: Filter by where you can move (compare value of cells).
                moves.push((x, y))
            }
        }
        moves
    }

    fn progress(&self) {
        for m in self.possible_moves() {
            if !self.previous_positions.contains(&m) {
            }
        }
    }
}

fn main() {
    let contents = include_str!("basic_input.txt");
    let height_map = HeightMap::parse(contents);
    while !height_map.finished() {
        height_map.progress()
    }
    println!("Move count: {}", height_map.move_count);
}
