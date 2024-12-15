fn calibrate(target: u128, nums: &[u128]) -> bool {
    match nums {
        [] => false,
        [num] => {
            num == &target
        }
        [num1, num2, end @ ..] => {
            calibrate(target, &[&[num1 + num2], end].concat()) || calibrate(target, &[&[num1 * num2], end].concat())
        }
    }
}

fn concalibrate(target: u128, nums: &[u128]) -> bool {
    match nums {
        [] => false,
        [num] => num == &target,
        [num1, num2, end @ ..] => {
            let add = concalibrate(target, &[&[num1 + num2], end].concat());
            let mul = concalibrate(target, &[&[num1 * num2], end].concat());
            let mut n1 = *num1;
            let mut n2 = *num2;
            while n2 > 0 {
                n1 *= 10;
                n2 /= 10;
            }
            let con = concalibrate(target, &[&[n1 + num2], end].concat());
            add || mul || con
        }
    }
}

pub fn solve1(data: &str) -> u128 {
    let mut res = 0;
    for line in data.lines() {
        let (target, calibration) = line.split_once(':').unwrap();
        let target: u128 = target.parse().unwrap();

        let calibration: Vec<u128> = calibration.split_whitespace().map(|n| n.parse().unwrap()).collect();

        if calibrate(target, &calibration) { res += target; }
    }
    println!("Day 7 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u128 {
    let mut res = 0;
    for line in data.lines() {
        let (target, calibration) = line.split_once(':').unwrap();
        let target: u128 = target.parse().unwrap();

        let calibration: Vec<u128> = calibration.split_whitespace().map(|n| n.parse().unwrap()).collect();

        if concalibrate(target, &calibration) { res += target; }
    }
    println!("Day 7 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    );

    assert_eq!(solve1(&data), 3749);
    assert_eq!(solve2(&data), 11387);
}