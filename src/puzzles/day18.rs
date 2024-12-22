use std::collections::VecDeque;

pub fn solve1<const N: usize, const BYTES: usize>(data: &str) -> usize {
    let mut grid: Vec<Vec<Option<char>>> = vec![vec![Some('.'); N]; N];

    let bytes = data.lines()
        .map(|line| {
            let (row, col) = line.split_once(',').unwrap();
            (row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
        });

    for (r, c) in bytes.into_iter().take(BYTES) {
        grid[r][c] = Some('#');
    }

    let mut res = 0;
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::from([(0, 0, 0)]);
    while let Some((r, c, steps)) = queue.pop_front() {
        if (r, c) == (N-1, N-1) {
            res = steps;
            break;
        }

        if grid[r][c].is_none() {
            continue;
        }

        grid[r][c] = None;
        if r > 0 && grid[r-1][c].is_some() && grid[r-1][c].unwrap() != '#' {
            queue.push_back((r-1, c, steps + 1));
        }
        if r < N-1 && grid[r+1][c].is_some() && grid[r+1][c].unwrap() != '#' {
            queue.push_back((r+1, c, steps + 1));
        }
        if c > 0 && grid[r][c-1].is_some() && grid[r][c-1].unwrap() != '#' {
            queue.push_back((r, c-1, steps + 1));
        }
        if c < N-1 && grid[r][c+1].is_some() && grid[r][c+1].unwrap() != '#' {
            queue.push_back((r, c+1, steps + 1));
        }
    }

    println!("Day 18 Part 1 = {res}");
    res
}

pub fn solve2<const N: usize>(data: &str) -> (usize, usize) {
    #[derive(Debug, Clone)]
    struct UnionFind {
        parents: Vec<Vec<(usize, usize)>>,
        rank: Vec<Vec<usize>>,
    }

    impl UnionFind {
        fn new(size: usize) -> Self {
            Self {
                parents: (0..size).map(|r| {
                    (0..size).map(|c| (r, c)).collect()
                }).collect(),
                rank: vec![vec![1; size]; size],
            }
        }

        fn find(&mut self, r: usize, c: usize) -> (usize, usize) {
            let (mut pr, mut pc) = (r, c);
            while (pr, pc) != self.parents[pr][pc] {
                self.parents[pr][pc] = self.parents[self.parents[pr][pc].0][self.parents[pr][pc].1];
                (pr, pc) = self.parents[pr][pc];
            }
            (pr, pc)
        }

        fn union(&mut self, (xr, xc): (usize, usize), (yr, yc): (usize, usize)) {
            let (px, py) = (self.find(xr, xc), self.find(yr, yc));
            if px == py { return; }
            if self.rank[px.0][px.1] > self.rank[py.0][py.1] {
                self.parents[py.0][py.1] = px;
                self.rank[px.0][px.1] += self.rank[py.0][py.1];
            } else {
                self.parents[px.0][px.1] = py;
                self.rank[py.0][py.1] += self.rank[px.0][px.1];
            }
        }
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; N]; N];
    let mut uf = UnionFind::new(N);

    let bytes: Vec<(usize, usize)> = data.lines()
        .map(|line| {
            let (row, col) = line.split_once(',').unwrap();
            (row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
        })
        .collect();

    for (r, c) in bytes.iter() {
        grid[*r][*c] = '#';
    }

    for (r, row) in grid.iter().enumerate() {
        for (c, &path) in row.iter().enumerate() {
            if path == '#' { continue; }

            if r > 0 && grid[r-1][c] != '#' { uf.union((r, c), (r-1, c));}
            if c > 0 && grid[r][c-1] != '#' { uf.union((r, c), (r, c-1));}
            if r < N - 1 && grid[r+1][c] != '#' { uf.union((r, c), (r+1, c));}
            if c < N - 1 && grid[r][c+1] != '#' { uf.union((r, c), (r, c+1));}
        }
    }

    let mut res = (0,0);
    for (r, c) in bytes.into_iter().rev() {
        grid[r][c] = '.';
        if r > 0 && grid[r-1][c] != '#' { uf.union((r, c), (r-1, c));}
        if c > 0 && grid[r][c-1] != '#' { uf.union((r, c), (r, c-1));}
        if r < N - 1 && grid[r+1][c] != '#' { uf.union((r, c), (r+1, c));}
        if c < N - 1 && grid[r][c+1] != '#' { uf.union((r, c), (r, c+1));}

        if uf.find(0, 0) == uf.find(N-1, N-1) { res = (r, c); break; }
    }

    println!("Day 18 Part 1 = {res:?}");
    res
}

#[test]
fn test() {
    let data = String::from(
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    );

    assert_eq!(solve1::<7, 12>(&data), 22);
    assert_eq!(solve2::<7>(&data), (6,1));
}