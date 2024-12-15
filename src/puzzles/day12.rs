use std::collections::HashSet;

pub fn solve1(data: &str) -> usize {
    let garden: Vec<Vec<char>> = data.lines().map(|line| {
        line.chars().collect()
    }).collect();

    fn dfs(
        r: usize, 
        c: usize, 
        plot: char,
        grid: &Vec<Vec<char>>, 
        seen: &mut HashSet<(usize, usize)>,
        perimeter: &mut usize, 
        area: &mut usize
    ) {
        let (rows, cols) = (grid.len(), grid[0].len());
        seen.insert((r,c));
        *area += 1;
        let mut p = 4;
        if r > 0 && grid[r-1][c] == plot { p -= 1;}
        if c > 0 && grid[r][c-1] == plot { p -= 1;}
        if r < rows - 1 && grid[r+1][c] == plot { p -= 1;}
        if c < cols - 1&& grid[r][c+1] == plot { p -= 1;}

        *perimeter += p;

        if r > 0 && !seen.contains(&(r-1,c)) && grid[r-1][c] == plot { dfs(r-1, c, plot, grid, seen, perimeter, area); }
        if c > 0 && !seen.contains(&(r,c-1)) && grid[r][c-1] == plot { dfs(r, c-1, plot, grid, seen, perimeter, area); }
        if r < rows - 1 && !seen.contains(&(r+1,c)) && grid[r+1][c] == plot { dfs(r+1, c, plot, grid, seen, perimeter, area); }
        if c < cols - 1 && !seen.contains(&(r,c+1)) && grid[r][c+1] == plot { dfs(r, c+1, plot, grid, seen, perimeter, area); }
    }

    let mut seen_garden: HashSet<(usize, usize)> = HashSet::new();
    let mut res = 0;

    for (r, row) in garden.iter().enumerate() {
        for (c, plot) in row.iter().enumerate() {
            if seen_garden.contains(&(r,c)) {continue;}

            let mut curr_seen: HashSet<(usize, usize)> = HashSet::new();
            let (mut area, mut perimeter) = (0, 0);
            dfs(r, c, *plot, &garden, &mut curr_seen, &mut perimeter, &mut area);

            // println!("Plot {plot} area = {area}, perimeter = {perimeter}, price = {}", perimeter * area);
            res += area * perimeter;
            seen_garden.extend(curr_seen.into_iter());
        }
    }

    println!("Day 12 Part 1 = {res}");
    res
}

struct Plot {
    variety: char,
    locs: HashSet<(usize, usize)>,
}

impl Plot {
    pub fn new(variety: char, r: usize, c: usize, rows: usize, cols: usize, grid: &mut Vec<Vec<Option<char>>>) -> Self {
        let mut new_plot = Self {
            variety,
            locs: HashSet::new(),
        };
        new_plot.dfs(r, c, rows, cols, grid);
        new_plot
    }

    fn dfs(
        &mut self,
        r: usize,
        c: usize,
        rows: usize,
        cols: usize,
        grid: &mut Vec<Vec<Option<char>>>,
    ) {
        self.locs.insert((r, c));
        grid[r][c] = None;

        if r > 0 && grid[r-1][c].is_some() && grid[r-1][c].unwrap() == self.variety { self.dfs(r-1, c, rows, cols, grid); }
        if c > 0 && grid[r][c-1].is_some() && grid[r][c-1].unwrap() == self.variety { self.dfs(r, c-1, rows, cols, grid); }
        if r < rows - 1 && grid[r+1][c].is_some() && grid[r+1][c].unwrap() == self.variety { self.dfs(r+1, c, rows, cols, grid); }
        if c < cols - 1 && grid[r][c+1].is_some() && grid[r][c+1].unwrap() == self.variety { self.dfs(r, c+1, rows, cols, grid); }
    }

    fn area(&self) -> usize {
        self.locs.len()
    }

    fn sides(&self) -> usize {
        let (mut rmin, mut rmax) = (usize::MAX, usize::MIN);
        let (mut cmin, mut cmax) = (usize::MAX, usize::MIN);

        for &(r, c) in self.locs.iter() {
            rmin = rmin.min(r);
            rmax = rmax.max(r);
            cmin = cmin.min(c);
            cmax = cmax.max(c);
        }

        let mut side_count = 0;
        // Top Sides
        for r in rmin..rmax + 1 {
            let mut cpos = cmin;
            while cpos < cmax + 1 {
                if !self.locs.contains(&(r, cpos)) || (r > 0 && self.locs.contains(&(r-1, cpos))) { cpos += 1; continue; }
                while cpos < cmax + 1 && self.locs.contains(&(r, cpos)) && (r == 0 || !self.locs.contains(&(r-1, cpos))) { cpos += 1; }
                side_count += 1;
            }
        }

        // Bottom Sides
        for r in rmin..rmax + 1 {
            let mut cpos = cmin;
            while cpos < cmax + 1 {
                if !self.locs.contains(&(r, cpos)) || (r < rmax && self.locs.contains(&(r+1, cpos))) { cpos += 1; continue; }
                while cpos < cmax + 1 && self.locs.contains(&(r, cpos)) && (r == rmax || !self.locs.contains(&(r+1, cpos))) { cpos += 1; }
                side_count += 1;
            }
        }

        // Left Sides
        for c in cmin..cmax + 1 {
            let mut rpos = rmin;
            while rpos < rmax + 1 {
                if !self.locs.contains(&(rpos, c)) || (c > 0 && self.locs.contains(&(rpos, c-1))) { rpos += 1; continue; }
                while rpos < rmax + 1 && self.locs.contains(&(rpos, c)) && (c == 0 || !self.locs.contains(&(rpos, c-1))) { rpos += 1; }
                side_count += 1;
            }
        }

        // Right Sides
        for c in cmin..cmax + 1 {
            let mut rpos = rmin;
            while rpos < rmax + 1 {
                if !self.locs.contains(&(rpos, c)) || (c < cmax && self.locs.contains(&(rpos, c+1))) { rpos += 1; continue; }
                while rpos < rmax + 1 && self.locs.contains(&(rpos, c)) && (c == cmax || !self.locs.contains(&(rpos, c+1))) { rpos += 1; }
                side_count += 1;
            }
        }

        side_count
    }
}

pub fn solve2(data: &str) -> usize {
    let mut garden: Vec<Vec<Option<char>>> = data.lines().map(|line| {
        line.chars().map(Some).collect()
    }).collect();

    let (rows, cols) = (garden.len(), garden[0].len());
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            if let Some(plot) = garden[r][c] {
                let curr_plot = Plot::new(plot, r, c, rows, cols, &mut garden);
                res += curr_plot.area()*curr_plot.sides();
            }
        }
    }

    println!("Day 12 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data1 = String::from(
"AAAA
BBCD
BBCC
EEEC"
    );
    let data2 = String::from(
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
    );

    let data3 = String::from(
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
    );

    let data4 = String::from(
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
    );

    let data5 = String::from(
"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
    );

    assert_eq!(solve1(&data1), 140);
    assert_eq!(solve1(&data2), 772);
    assert_eq!(solve1(&data3), 1930);

    assert_eq!(solve2(&data1), 80);
    assert_eq!(solve2(&data2), 436);
    assert_eq!(solve2(&data3), 1206);
    assert_eq!(solve2(&data4), 236); 
    assert_eq!(solve2(&data5), 368);
}