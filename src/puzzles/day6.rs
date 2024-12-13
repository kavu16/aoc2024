#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

pub fn solve1(data: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut dir = Direction::Up;

    let (mut r, mut c) = (0, 0);
    for (ri, row) in grid.iter().enumerate() {
        for (ci, val) in row.iter().enumerate() {
            if *val == '^' { (r, c) = (ri, ci); break; }
        }
    }

    let mut res = 1;
    while (1..rows-1).contains(&r) && (1..cols-1).contains(&c) {
        if grid[r][c] != 'X' {
            grid[r][c] = 'X';
            res += 1;
        }
        match dir {
            Direction::Up => {
                if grid[r-1][c] == '#' {
                    dir = dir.next();
                    c += 1;
                } else {
                    r -= 1;
                }
            }
            Direction::Down => {
                if grid[r+1][c] == '#' {
                    dir = dir.next();
                    c -= 1;
                } else {
                    r += 1;
                }
            }
            Direction::Left => {
                if grid[r][c-1] == '#' {
                    dir = dir.next();
                    r -= 1;
                } else {
                    c -= 1;
                }
            }
            Direction::Right => {
                if grid[r][c+1] == '#' {
                    dir = dir.next();
                    r += 1;
                } else {
                    c += 1;
                }
            }
        }
    }

    println!("Day 6 Part 1 = {res}");
    res
}

use std::collections::HashSet;

pub fn solve2(data: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut dir = Direction::Up;

    let (mut r, mut c) = (0, 0);
    for (ri, row) in grid.iter().enumerate() {
        for (ci, val) in row.iter().enumerate() {
            if *val == '^' { (r, c) = (ri, ci); break; }
        }
    }

    let mut res = 0;
    let mut seen: HashSet<(Direction, (usize, usize))> = HashSet::new();
    while (1..rows-1).contains(&r) && (1..cols-1).contains(&c) {
        if seen.contains(&(dir.next(), (r,c))) { res += 1;}
        seen.insert((dir, (r,c)));
        match dir {
            Direction::Up => {
                if grid[r-1][c] == '#' {
                    dir = dir.next();
                    c += 1;
                } else {
                    r -= 1;
                }
            }
            Direction::Down => {
                if grid[r+1][c] == '#' {
                    dir = dir.next();
                    c -= 1;
                } else {
                    r += 1;
                }
            }
            Direction::Left => {
                if grid[r][c-1] == '#' {
                    dir = dir.next();
                    r -= 1;
                } else {
                    c -= 1;
                }
            }
            Direction::Right => {
                if grid[r][c+1] == '#' {
                    dir = dir.next();
                    r += 1;
                } else {
                    c += 1;
                }
            }
        }
    }

    println!("Day 6 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    );

    assert_eq!(solve1(&data), 41);
    assert_eq!(solve2(&data), 6);
}