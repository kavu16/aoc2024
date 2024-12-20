use itertools::Itertools;

#[derive(Debug)]
struct Robot {
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn patrol(&self, time: i64, xwidth: i64, ywidth: i64) -> (i64, i64) {
        let px = self.px + self.vx * time;
        let py = self.py + self.vy * time;
        (px.rem_euclid(xwidth), py.rem_euclid(ywidth))
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (px, py, vx, vy) = value.split(&[' ', ',', '='])
            .flat_map(|n| n.parse::<i64>())
            .next_tuple()
            .unwrap();

        Self {
            px,
            py,
            vx,
            vy
        }
    }
}

pub fn solve1<const X: i64, const Y: i64, const T: i64>(data: &str) -> i64 {
    let (midx, midy) = (X/2, Y/2);
    let robots: Vec<Robot> = data.lines().map(Robot::from).collect();
    let mut safety = [0, 0, 0, 0];
    robots.into_iter()
        .map(|r| {
            r.patrol(T, X, Y)
        })
        .for_each(|(px, py)| {
            if px != midx && py != midy {
                safety[(px < midx) as usize + 2*(py < midy) as usize] += 1;
            }
        });

    let res = safety.into_iter().product();
    println!("Day 14 Part 1 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
    );

    assert_eq!(solve1::<11, 7, 100>(&data), 12);
}