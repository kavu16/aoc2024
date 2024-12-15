use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve1(data: &str) -> u32 {
    let grid: Vec<Vec<u8>> = data.lines().map(|line| {
        line.chars().map(|n| n.to_digit(10).unwrap() as u8).collect()
    }).collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut trail_heads: HashMap<u8, HashSet<(usize, usize)>> = HashMap::new();
    let mut queue: VecDeque<(u8, (usize, usize))> = VecDeque::new();

    let mut id = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 0 {
                trail_heads.insert(id, HashSet::new());
                queue.push_back((id, (r, c)));
                id += 1;
            }
        }
    }

    while let Some((id, (r, c))) = queue.pop_front() {
        let curr_height = grid[r][c];

        if curr_height == 9 {
            trail_heads.get_mut(&id).unwrap().insert((r, c));
            continue;
        }

        if r > 0 && grid[r-1][c] == curr_height + 1 {
            queue.push_back((id, (r-1, c)));
        }
        if c > 0 && grid[r][c-1] == curr_height + 1 {
            queue.push_back((id, (r, c-1)));
        }
        if r < rows - 1 && grid[r+1][c] == curr_height + 1 {
            queue.push_back((id, (r+1, c)));
        }
        if c < cols - 1 && grid[r][c+1] == curr_height + 1 {
            queue.push_back((id, (r, c+1)));
        }
    }
    let res = trail_heads.into_values().map(|peaks| peaks.len() as u32).sum();

    println!("Day 10 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u32 {
    let grid: Vec<Vec<u8>> = data.lines().map(|line| {
        line.chars().map(|n| n.to_digit(10).unwrap() as u8).collect()
    }).collect();

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut trail_heads: HashMap<u8, u32> = HashMap::new();
    let mut queue: VecDeque<(u8, (usize, usize))> = VecDeque::new();

    let mut id = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 0 {
                trail_heads.insert(id, 0);
                queue.push_back((id, (r, c)));
                id += 1;
            }
        }
    }

    while let Some((id, (r, c))) = queue.pop_front() {
        let curr_height = grid[r][c];

        if curr_height == 9 {
            *trail_heads.get_mut(&id).unwrap() += 1;
            continue;
        }

        if r > 0 && grid[r-1][c] == curr_height + 1 {
            queue.push_back((id, (r-1, c)));
        }
        if c > 0 && grid[r][c-1] == curr_height + 1 {
            queue.push_back((id, (r, c-1)));
        }
        if r < rows - 1 && grid[r+1][c] == curr_height + 1 {
            queue.push_back((id, (r+1, c)));
        }
        if c < cols - 1 && grid[r][c+1] == curr_height + 1 {
            queue.push_back((id, (r, c+1)));
        }
    }
    let res = trail_heads.into_values().sum();

    println!("Day 10 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    );

    assert_eq!(solve1(&data), 36);
    assert_eq!(solve2(&data), 81);
}