use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

// impl Direction {
//     fn turn(&self, clockwise: bool) -> Self {
//         match self {
//             Direction::North => if clockwise { Direction::East } else { Direction::West },
//             Direction::East => if clockwise { Direction::South } else { Direction::North },
//             Direction::South => if clockwise { Direction::West } else { Direction::East },
//             Direction::West => if clockwise { Direction::North } else { Direction::South },
//         }
//     }
// }

pub fn solve1(data: &str) -> usize {
    let grid: Vec<Vec<char>> = data.lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();
    
    let mut start = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, &pos) in row.iter().enumerate() {
            if pos == 'S' { start = (r, c); break; }
        }
    }

    let mut queue: BinaryHeap<(Reverse<usize>, Direction, usize, usize)> = BinaryHeap::from([(Reverse(0), Direction::East, start.0, start.1)]);
    let mut cache: HashMap<(Direction, usize, usize), usize> = HashMap::new();
    let mut res = usize::MAX;

    while let Some((Reverse(curr_score), curr_dir, curr_r, curr_c)) = queue.pop() {
        // println!("{curr_dir:?}, ({curr_r}, {curr_c}), {curr_score}");
        if grid[curr_r][curr_c] == 'E' { 
            
            res = res.min(curr_score); 
            break; 
        }

        cache.insert((curr_dir, curr_r, curr_c), curr_score);

        match curr_dir {
            Direction::North => {
                if grid[curr_r-1][curr_c] != '#' && *cache.entry((curr_dir, curr_r-1, curr_c)).or_insert(usize::MAX) > curr_score + 1 {
                    queue.push((Reverse(curr_score+1), Direction::North, curr_r-1, curr_c));
                }
                if grid[curr_r][curr_c+1] != '#' && *cache.entry((Direction::East, curr_r, curr_c+1)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::East, curr_r, curr_c+1));
                }
                if grid[curr_r][curr_c-1] != '#' && *cache.entry((Direction::West, curr_r, curr_c-1)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::West, curr_r, curr_c-1));
                }
            }
            Direction::East => {
                if grid[curr_r][curr_c+1] != '#' && *cache.entry((curr_dir, curr_r, curr_c+1)).or_insert(usize::MAX) > curr_score + 1 {
                    queue.push((Reverse(curr_score+1), Direction::East, curr_r, curr_c+1));
                }
                if grid[curr_r+1][curr_c] != '#' && *cache.entry((Direction::South, curr_r+1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::South, curr_r+1, curr_c));
                }
                if grid[curr_r-1][curr_c] != '#' && *cache.entry((Direction::North, curr_r-1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::North, curr_r-1, curr_c));
                }
            }
            Direction::South => {
                if grid[curr_r+1][curr_c] != '#' && *cache.entry((curr_dir, curr_r+1, curr_c)).or_insert(usize::MAX) > curr_score + 1 {
                    queue.push((Reverse(curr_score+1), Direction::South, curr_r+1, curr_c));
                }
                if grid[curr_r][curr_c+1] != '#' && *cache.entry((Direction::East, curr_r, curr_c+1)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::East, curr_r, curr_c+1));
                }
                if grid[curr_r][curr_c-1] != '#' && *cache.entry((Direction::West, curr_r, curr_c-1)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::West, curr_r, curr_c-1));
                }
            }
            Direction::West => {
                if grid[curr_r][curr_c-1] != '#' && *cache.entry((curr_dir, curr_r, curr_c-1)).or_insert(usize::MAX) > curr_score + 1 {
                    queue.push((Reverse(curr_score+1), Direction::West, curr_r, curr_c-1));
                }
                if grid[curr_r+1][curr_c] != '#' && *cache.entry((Direction::South, curr_r+1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::South, curr_r+1, curr_c));
                }
                if grid[curr_r-1][curr_c] != '#' && *cache.entry((Direction::North, curr_r-1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    queue.push((Reverse(curr_score+1001), Direction::North, curr_r-1, curr_c));
                }
            }
        }
    }
    
    println!("Day 16 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let grid: Vec<Vec<char>> = data.lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();
    
    let mut start = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, &pos) in row.iter().enumerate() {
            if pos == 'S' { start = (r, c); break; }
        }
    }

    let mut queue: BinaryHeap<(Reverse<usize>, Direction, usize, usize, Vec<(usize, usize)>)> = BinaryHeap::from([(Reverse(0), Direction::East, start.0, start.1, vec![start])]);
    let mut cache: HashMap<(Direction, usize, usize), usize> = HashMap::new();
    let mut shortest_path = usize::MAX;
    let mut paths:  HashSet<(usize, usize)> = HashSet::new();

    while let Some((Reverse(curr_score), curr_dir, curr_r, curr_c, curr_path)) = queue.pop() {
        // println!("{curr_dir:?}, ({curr_r}, {curr_c}), {curr_score}");
        if grid[curr_r][curr_c] == 'E' { 
            if curr_score == shortest_path {
                paths.extend(curr_path.into_iter());
            } else if curr_score < shortest_path {
                shortest_path = curr_score;
                paths.clear();
                paths.extend(curr_path.into_iter());
            }

            continue;
        }

        cache.insert((curr_dir, curr_r, curr_c), curr_score);

        match curr_dir {
            Direction::North => {
                if grid[curr_r-1][curr_c] != '#' && *cache.entry((curr_dir, curr_r-1, curr_c)).or_insert(usize::MAX) > curr_score + 1 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r-1, curr_c));
                    queue.push((Reverse(curr_score+1), Direction::North, curr_r-1, curr_c, nxt_path));
                }
                if grid[curr_r][curr_c+1] != '#' && *cache.entry((Direction::East, curr_r, curr_c+1)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r, curr_c+1));
                    queue.push((Reverse(curr_score+1001), Direction::East, curr_r, curr_c+1, nxt_path));
                }
                if grid[curr_r][curr_c-1] != '#' && *cache.entry((Direction::West, curr_r, curr_c-1)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r, curr_c-1));
                    queue.push((Reverse(curr_score+1001), Direction::West, curr_r, curr_c-1, nxt_path));
                }
            }
            Direction::East => {
                if grid[curr_r][curr_c+1] != '#' && *cache.entry((curr_dir, curr_r, curr_c+1)).or_insert(usize::MAX) > curr_score + 1 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r, curr_c+1));
                    queue.push((Reverse(curr_score+1), Direction::East, curr_r, curr_c+1, nxt_path));
                }
                if grid[curr_r+1][curr_c] != '#' && *cache.entry((Direction::South, curr_r+1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r+1, curr_c));
                    queue.push((Reverse(curr_score+1001), Direction::South, curr_r+1, curr_c, nxt_path));
                }
                if grid[curr_r-1][curr_c] != '#' && *cache.entry((Direction::North, curr_r-1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r-1, curr_c));
                    queue.push((Reverse(curr_score+1001), Direction::North, curr_r-1, curr_c, nxt_path));
                }
            }
            Direction::South => {
                if grid[curr_r+1][curr_c] != '#' && *cache.entry((curr_dir, curr_r+1, curr_c)).or_insert(usize::MAX) > curr_score + 1 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r+1, curr_c));
                    queue.push((Reverse(curr_score+1), Direction::South, curr_r+1, curr_c, nxt_path));
                }
                if grid[curr_r][curr_c+1] != '#' && *cache.entry((Direction::East, curr_r, curr_c+1)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r, curr_c+1));
                    queue.push((Reverse(curr_score+1001), Direction::East, curr_r, curr_c+1, nxt_path));
                }
                if grid[curr_r][curr_c-1] != '#' && *cache.entry((Direction::West, curr_r, curr_c-1)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r, curr_c-1));
                    queue.push((Reverse(curr_score+1001), Direction::West, curr_r, curr_c-1, nxt_path));
                }
            }
            Direction::West => {
                if grid[curr_r][curr_c-1] != '#' && *cache.entry((curr_dir, curr_r, curr_c-1)).or_insert(usize::MAX) > curr_score + 1 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r, curr_c-1));
                    queue.push((Reverse(curr_score+1), Direction::West, curr_r, curr_c-1, nxt_path));
                }
                if grid[curr_r+1][curr_c] != '#' && *cache.entry((Direction::South, curr_r+1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r+1, curr_c));
                    queue.push((Reverse(curr_score+1001), Direction::South, curr_r+1, curr_c, nxt_path));
                }
                if grid[curr_r-1][curr_c] != '#' && *cache.entry((Direction::North, curr_r-1, curr_c)).or_insert(usize::MAX) > curr_score + 1001 {
                    let mut nxt_path = curr_path.clone();
                    nxt_path.push((curr_r-1, curr_c));
                    queue.push((Reverse(curr_score+1001), Direction::North, curr_r-1, curr_c, nxt_path));
                }
            }
        }
    }
    let res = paths.len();
    println!("Day 16 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
    );

    let data2 = String::from(
"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
    );

    assert_eq!(solve1(&data), 7036);
    assert_eq!(solve1(&data2), 11048);

    assert_eq!(solve2(&data), 45);
    assert_eq!(solve2(&data2), 64);
}