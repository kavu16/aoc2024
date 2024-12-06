use regex::Regex;

pub fn solve1(data: &String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // let mut nums: Vec<(i32, i32)> = vec![];

    // for (_, [n1, n2]) in re.captures_iter(data).map(|c| c.extract()) {
    //     nums.push((n1.parse().unwrap(), n2.parse().unwrap()));
    // }
    // let res = nums.into_iter().map(|(n1, n2)| n1 * n2).sum();

    let res = re.captures_iter(data)
        .map(|c| c.extract())
        .map(|(_, [n1, n2])| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
        .sum();
    println!("Day 3 Part 1 = {res}");
    res
}

pub fn solve2(data: &String) -> i32 {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut do_toggle = true;
    let mut res = 0;
    for m in re.find_iter(data) {
        let m = m.as_str();
        // println!("match: {m}");
        if &m[0..4] == "do()" { 
            do_toggle = true;
        } else if &m[0..3] == "mul" {
            if !do_toggle { continue; }
            let (l, r) = m[3..].split_once(',').unwrap();
            let l = l[1..].parse::<i32>().unwrap();
            let r = r[..(r.len() - 1)].parse::<i32>().unwrap();
            res += r * l;
        } else {
            do_toggle = false;
        }
    }
    println!("Day 3 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    );

    let data2 = String::from(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    );

    assert_eq!(solve1(&data), 161);
    assert_eq!(solve2(&data2), 48)
}