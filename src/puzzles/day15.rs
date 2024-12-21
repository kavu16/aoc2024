use std::collections::VecDeque;

pub fn solve1(data: &str) -> usize {
    let (grid, moves) = data.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = grid.lines()
        .map(|line| line.chars().collect())
        .collect();

    let moves = moves.chars().filter(|&c| c != '\n');
    let mut curr = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char == '@' { curr = (r as i32, c as i32); break; }
        }
    }

    for robot_move in moves {
        let (mut r, mut c) = curr;
        let (dr, dc) = match robot_move {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            m => panic!("Unrecognized move: {m}"),
        };
        r += dr;
        c += dc;

        while grid[r as usize][c as usize] == 'O' {
            r += dr;
            c += dc;
        }
        if grid[r as usize][c as usize] == '.' {
            grid[r as usize][c as usize] = 'O';
            grid[curr.0 as usize][curr.1 as usize] = '.';
            grid[(curr.0 + dr) as usize][(curr.1 + dc) as usize] = '@';
            curr = (curr.0 + dr, curr.1 + dc);
        }
    }

    // for row in grid.iter() {
    //     println!("{row:?}");
    // }
    
    let res = grid.into_iter()
        .enumerate()
        .map(|(r, row)| {
            row.into_iter().enumerate()
                .map(|(c, char)| {
                    if char == 'O' {
                        100 * r + c
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();



    println!("Day 15 Part 2 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let (grid, moves) = data.split_once("\n\n").unwrap();

    let mut grid: Vec<Vec<char>> = grid.lines()
        .map(|line| {
            line.chars().flat_map(|c| {
                match c {
                    '#' => ['#', '#'],
                    '@' => ['@', '.'],
                    '.' => ['.', '.'],
                    'O' => ['[', ']'],
                    _ => panic!("unrecognized grid mark"),
                }
            }).collect()
        })
        .collect();

    let moves = moves.chars().filter(|&c| c != '\n');
    let mut curr = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char == '@' { curr = (r, c); break; }
        }
    }

    fn north_move(row: usize, left: usize, right: usize, grid: &mut Vec<Vec<char>>) {
        if grid[row-1][left] == '#' || grid[row-1][right] == '#' { return; }
        let mut seen: VecDeque<(usize, usize, usize)> = VecDeque::new();
        seen.push_back((row, left, right));

        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        if grid[row-1][left] == ']' { queue.push_back((row-1, left-1, left)); }
        if grid[row-1][right] == '[' { queue.push_back((row-1, right, right+1)); }
        if grid[row-1][left] == '[' && grid[row-1][right] == ']' { queue.push_back((row-1, left, right)); }

        while let Some((curr_row, curr_l, curr_r)) = queue.pop_front() {
            if grid[curr_row-1][curr_l] == '#' || grid[curr_row-1][curr_r] == '#' { return; }

            seen.push_back((curr_row, curr_l, curr_r));

            if grid[curr_row-1][curr_l] == ']' { queue.push_back((curr_row-1, curr_l-1, curr_l)); }
            if grid[curr_row-1][curr_r] == '[' { queue.push_back((curr_row-1, curr_r, curr_r+1)); }
            if grid[curr_row-1][curr_l] == '[' && grid[curr_row-1][curr_r] == ']' { queue.push_back((curr_row-1, curr_l, curr_r)); } 
        }

        while let Some((curr_row, curr_l, curr_r)) = seen.pop_back() {
            grid[curr_row][curr_l] = '.';
            grid[curr_row][curr_r] = '.';
            grid[curr_row-1][curr_l] = '[';
            grid[curr_row-1][curr_r] = ']';
        }
    }

    fn south_move(row: usize, left: usize, right: usize, grid: &mut Vec<Vec<char>>) {
        if grid[row+1][left] == '#' || grid[row+1][right] == '#' { return; }
        let mut seen: VecDeque<(usize, usize, usize)> = VecDeque::new();
        seen.push_back((row, left, right));

        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        if grid[row+1][left] == ']' { queue.push_back((row+1, left-1, left)); }
        if grid[row+1][right] == '[' { queue.push_back((row+1, right, right+1)); }
        if grid[row+1][left] == '[' && grid[row+1][right] == ']' { queue.push_back((row+1, left, right)); }

        while let Some((curr_row, curr_l, curr_r)) = queue.pop_front() {
            if grid[curr_row+1][curr_l] == '#' || grid[curr_row+1][curr_r] == '#' { return; }

            seen.push_back((curr_row, curr_l, curr_r));

            if grid[curr_row+1][curr_l] == ']' { queue.push_back((curr_row+1, curr_l-1, curr_l)); }
            if grid[curr_row+1][curr_r] == '[' { queue.push_back((curr_row+1, curr_r, curr_r+1)); }
            if grid[curr_row+1][curr_l] == '[' && grid[curr_row+1][curr_r] == ']' { queue.push_back((curr_row+1, curr_l, curr_r)); } 
        }

        while let Some((curr_row, curr_l, curr_r)) = seen.pop_back() {
            grid[curr_row][curr_l] = '.';
            grid[curr_row][curr_r] = '.';
            grid[curr_row+1][curr_l] = '[';
            grid[curr_row+1][curr_r] = ']';
        }
    }

    for robot_move in moves {
        let (r, c) = curr;

        match robot_move {
            '^' => {
                if grid[r-1][c] == '[' { north_move(r-1, c, c+1, &mut grid); }
                if grid[r-1][c] == ']' { north_move(r-1, c-1, c, &mut grid); }
                if grid[r-1][c] == '.' { grid[r-1][c] = '@'; grid[r][c] = '.'; curr = (r-1, c); }
            }
            '>' => {
                let mut curr_c = c+1;
                while grid[r][curr_c] == '[' || grid[r][curr_c] == ']' { curr_c += 1; }
                if grid[r][curr_c] == '.' {
                    while curr_c > c {
                        grid[r][curr_c] = grid[r][curr_c-1];
                        curr_c -= 1;
                    }
                    grid[r][c] = '.';
                    curr = (r, c+1);
                }
            }
            'v' => {
                if grid[r+1][c] == '[' { south_move(r+1, c, c+1, &mut grid); }
                if grid[r+1][c] == ']' { south_move(r+1, c-1, c, &mut grid); }
                if grid[r+1][c] == '.' { grid[r+1][c] = '@'; grid[r][c] = '.'; curr = (r+1, c); }
            }
            '<' => {
                let mut curr_c = c-1;
                while grid[r][curr_c] == '[' || grid[r][curr_c] == ']' { curr_c -= 1; }
                if grid[r][curr_c] == '.' {
                    while curr_c < c {
                        grid[r][curr_c] = grid[r][curr_c+1];
                        curr_c += 1;
                    }
                    grid[r][c] = '.';
                    curr = (r, c-1);
                }
            }
            m => panic!("Unrecognized move: {m}"),
        }
    }

    // for row in grid.iter() {
    //     println!("{}", row.iter().collect::<String>());
    // }
    
    let res = grid.into_iter()
        .enumerate()
        .map(|(r, row)| {
            row.into_iter().enumerate()
                .map(|(c, char)| {
                    if char == '[' {
                        100 * r + c
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum();

    println!("Day 15 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let small = String::from(
"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
    );

    let bigger = String::from(
"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
    );

    assert_eq!(solve1(&small), 2028);
    assert_eq!(solve1(&bigger), 10092);

    assert_eq!(solve2(&bigger), 9021);
}