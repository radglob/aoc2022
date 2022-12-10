type Grid = Vec<Vec<u32>>;

fn parse_grid(input: &str) -> Grid {
    let mut g = vec![];
    for (idx, line) in input.lines().enumerate() {
        g.push(vec![]);
        for c in line.chars() {
            let n = c.to_digit(10).unwrap();
            g[idx].push(n);
        }
    }
    g
}

fn visible_top(grid: &Grid, x: usize, y: usize) -> bool {
    let height = grid[x][y];
    (0..x).all(|idx| grid[idx][y] < height)
}

fn visible_bottom(grid: &Grid, x: usize, y: usize) -> bool {
    let height = grid[x][y];
    (x + 1..grid.len()).all(|idx| grid[idx][y] < height)
}

fn visible_right(grid: &Grid, x: usize, y: usize) -> bool {
    let height = grid[x][y];
    (y + 1..grid[x].len()).all(|idx| grid[x][idx] < height)
}

fn visible_left(grid: &Grid, x: usize, y: usize) -> bool {
    let height = grid[x][y];
    (0..y).all(|idx| grid[x][idx] < height)
}

fn visible(grid: &Grid, x: usize, y: usize) -> bool {
    let max_x: usize = grid.len() - 1;
    let max_y: usize = grid.first().unwrap().len() - 1;
    match (x, y) {
        (x, _) if x == 0 || x == max_x => true,
        (_, y) if y == 0 || y == max_y => true,
        _ => {
            visible_top(grid, x, y)
                || visible_bottom(grid, x, y)
                || visible_left(grid, x, y)
                || visible_right(grid, x, y)
        }
    }
}

fn viewing_distance_top(grid: &Grid, x: usize, y: usize) -> u32 {
    let height = grid[x][y];
    let mut distance = 0;
    for i in (x-1)..=0 {
        distance += 1;
        if height <= grid[i][y] {
            break;
        }
    }
    distance
}

fn viewing_distance_bottom(grid: &Grid, x: usize, y: usize) -> u32 {
    let height = grid[x][y];
    let mut distance = 0;
    for i in (x+1)..grid.len() {
        distance += 1;
        if height <= grid[i][y] {
            break;
        }
    }
    distance
}

fn viewing_distance_left(grid: &Grid, x: usize, y: usize) -> u32 {
    let height = grid[x][y];
    let mut distance = 0;
    for j in (y-1)..=0 {
        distance += 1;
        if height <= grid[x][j] {
            break;
        }
    }
    distance
}

fn viewing_distance_right(grid: &Grid, x: usize, y: usize) -> u32 {
    let height = grid[x][y];
    let mut distance = 0;
    for j in (y+1)..grid[0].len() {
        distance += 1;
        if height <= grid[x][j] {
            break;
        }
    }
    distance
}

fn viewing_distance(grid: &Grid, x: usize, y: usize) -> u32 {
    match (x, y) {
        (x, _) if x == 0 || x == grid.len() - 1 => 0,
        (_, y) if y == 0 || y == grid[0].len() - 1 => 0,
        _ => {
            viewing_distance_left(grid, x, y)
                * viewing_distance_right(grid, x, y)
                * viewing_distance_top(grid, x, y)
                * viewing_distance_bottom(grid, x, y)
        }
    }
}

fn main() {
    let contents = include_str!("basic_input.txt");
    let grid = parse_grid(contents);
    let mut max_viewing_distance = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let distance = viewing_distance(&grid, x, y);
            println!("Viewing distance from ({},{}) is {}", x, y, distance);
            if distance > max_viewing_distance {
                max_viewing_distance = distance
            }
        }
    }
    println!("{:?}", max_viewing_distance);
}
