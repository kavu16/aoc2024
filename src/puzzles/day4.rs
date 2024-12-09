pub fn solve1(data: &String) -> i32 {
    let xmas_grid: Vec<Vec<char>> = data.lines().map(|row| row.chars().collect()).collect();
    let (rows, cols) = (xmas_grid.len(), xmas_grid[0].len());
    let xmas = ['X', 'M', 'A', 'S'];

    let res = xmas_grid.iter().enumerate().map(|(r, row)| {
        row.iter()
            .enumerate()
            .filter(|(_c, &char)| char == 'X')
            .map(|(c, _char)| {
                let mut xmas_count = 0;
                // Go to right
                if c < cols - 3 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r][c+i] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Go to left
                if c > 2 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r][c-i] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Go up
                if r > 2 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r-i][c] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Go down
                if r < rows - 3 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r+i][c] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Up and right
                if r > 2 && c < cols - 3 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r-i][c+i] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Down and right
                if r < rows - 3 && c < cols - 3 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r+i][c+i] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Up and left
                if r > 2 && c > 2 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r-i][c-i] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }

                // Down and left
                if r < rows - 3 && c > 2 {
                    let mut xmasfound = true;
                    for i in 1..=3 {
                        if xmas_grid[r+i][c-i] != xmas[i] { xmasfound = false; break; }
                    }
                    xmas_count += xmasfound as i32;
                }
                xmas_count
            }).sum::<i32>()
    }).sum();

    println!("Day 4 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> i32 {
    let xmas_grid: Vec<Vec<char>> = data.lines().map(|row| row.chars().collect()).collect();
    let (rows, cols) = (xmas_grid.len(), xmas_grid[0].len());
    // let xmas = ['X', 'M', 'A', 'S'];

    let res = xmas_grid.iter().enumerate().map(|(r, row)| {
        row.iter()
            .enumerate()
            .filter(|(c, &char)| char == 'A' && (1..rows-1).contains(&r) && (1..cols-1).contains(c))
            .map(|(c, _char)| {
                let mut x_mas_count = 0;
                if (xmas_grid[r-1][c-1] == 'M' && xmas_grid[r-1][c+1] == 'M') && (xmas_grid[r+1][c-1] == 'S' && xmas_grid[r+1][c+1] == 'S') { x_mas_count += 1; }
                if (xmas_grid[r-1][c-1] == 'M' && xmas_grid[r-1][c+1] == 'S') && (xmas_grid[r+1][c-1] == 'M' && xmas_grid[r+1][c+1] == 'S') { x_mas_count += 1; }
                if (xmas_grid[r-1][c-1] == 'S' && xmas_grid[r-1][c+1] == 'S') && (xmas_grid[r+1][c-1] == 'M' && xmas_grid[r+1][c+1] == 'M') { x_mas_count += 1; }
                if (xmas_grid[r-1][c-1] == 'S' && xmas_grid[r-1][c+1] == 'M') && (xmas_grid[r+1][c-1] == 'S' && xmas_grid[r+1][c+1] == 'M') { x_mas_count += 1; }
                
                x_mas_count
            }).sum::<i32>()
    }).sum();

    println!("Day 4 Part 1 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    );

    assert_eq!(solve1(&data), 18);
    assert_eq!(solve2(&data), 9);
}