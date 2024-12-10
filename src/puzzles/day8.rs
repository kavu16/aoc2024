use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve1(data: &str) -> i32 {
    let grid: Vec<Vec<char>> = data.lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &antenna) in row.iter().enumerate() {
            if antenna == '.' {continue;}
            antennae.entry(antenna).or_default().push((r as i32, c as i32));
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_antenna, locations) in antennae {
        locations.into_iter().permutations(2).for_each(|pair| {
            let ((ra, ca), (rb, cb)) = (pair[0], pair[1]);
            let (rdiff, cdiff) = (ra - rb, ca - cb);
            if (0..=rows-1).contains(&(rb - rdiff)) && (0..=cols-1).contains(&(cb-cdiff)) {
                antinodes.insert((rb - rdiff, cb - cdiff));
            }
        });
    }
    let res = antinodes.len() as i32;
    println!("Day 8 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> i32 {
    let grid: Vec<Vec<char>> = data.lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &antenna) in row.iter().enumerate() {
            if antenna == '.' {continue;}
            antennae.entry(antenna).or_default().push((r as i32, c as i32));
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_antenna, locations) in antennae {
        locations.into_iter().permutations(2).for_each(|pair| {
            let ((ra, ca), (rb, cb)) = (pair[0], pair[1]);
            let (rdiff, cdiff) = (ra - rb, ca - cb);
            let mut i = 0;
            while (0..=rows-1).contains(&(rb - i * rdiff)) && (0..=cols-1).contains(&(cb - i * cdiff)) {
                antinodes.insert((rb - i * rdiff, cb - i * cdiff));
                i += 1;
            }
        });
    }
    let res = antinodes.len() as i32;
    println!("Day 8 Part 1 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    );

    assert_eq!(solve1(&data), 14);
    assert_eq!(solve2(&data), 34);
}