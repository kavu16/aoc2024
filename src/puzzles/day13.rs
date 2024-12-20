pub fn solve1(data: &str) -> i64 {
    let mut res = 0;
    for claw in data.split("\n\n") {
        let settings = claw.lines().collect::<Vec<_>>();
        let (l, r) = settings[0].split_once(',').unwrap();
        let ((_, x1), (_, y1)) = (l.split_once('+').unwrap(), r.split_once('+').unwrap());
        let (x1, y1) = (x1.parse::<i64>().unwrap(), y1.parse::<i64>().unwrap());

        let (l, r) = settings[1].split_once(',').unwrap();
        let ((_, x2), (_, y2)) = (l.split_once('+').unwrap(), r.split_once('+').unwrap());
        let (x2, y2) = (x2.parse::<i64>().unwrap(), y2.parse::<i64>().unwrap());

        let (l, r) = settings[2].split_once(',').unwrap();
        let ((_, x), (_, y)) = (l.split_once('=').unwrap(), r.split_once('=').unwrap());
        let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());

        let d = x1 * y2 - y1 * x2;
        let da = x * y2 - y * x2;
        let db = y * x1 - x * y1;

        res += if da % d == 0 && db % d == 0 { 3 * da / d + db / d} else { 0 };
    }
    
    println!("Day 13 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> i64 {
    let mut res = 0;
    for claw in data.split("\n\n") {
        let settings = claw.lines().collect::<Vec<_>>();
        let (l, r) = settings[0].split_once(',').unwrap();
        let ((_, x1), (_, y1)) = (l.split_once('+').unwrap(), r.split_once('+').unwrap());
        let (x1, y1) = (x1.parse::<i64>().unwrap(), y1.parse::<i64>().unwrap());

        let (l, r) = settings[1].split_once(',').unwrap();
        let ((_, x2), (_, y2)) = (l.split_once('+').unwrap(), r.split_once('+').unwrap());
        let (x2, y2) = (x2.parse::<i64>().unwrap(), y2.parse::<i64>().unwrap());

        let (l, r) = settings[2].split_once(',').unwrap();
        let ((_, x), (_, y)) = (l.split_once('=').unwrap(), r.split_once('=').unwrap());
        let (x, y) = (x.parse::<i64>().unwrap() + 10_000_000_000_000, y.parse::<i64>().unwrap() + 10_000_000_000_000);

        let d = x1 * y2 - y1 * x2;
        let da = x * y2 - y * x2;
        let db = y * x1 - x * y1;

        res += if da % d == 0 && db % d == 0 { 3 * da / d + db / d} else { 0 };
    }
    
    println!("Day 13 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
    );

    assert_eq!(solve1(&data), 480);
}