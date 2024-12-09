use itertools::Itertools;

pub fn solve1(data: &String) -> usize {
    let mut res = 0;
    for line in data.lines() {
        let diffs = line.split_whitespace().tuple_windows()
            .map(|(n1, n2)| n1.parse::<i32>().unwrap() - n2.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        
        res += diffs.iter().tuple_windows()
                    .all(|(d1, d2)| d1 * d2  > 0 && (1..=3).contains(&d1.abs()) && (1..=3).contains(&d2.abs())) 
                    as usize;
    }
    println!("Day 2 Part 1 = {res}");
    res
}

pub fn solve2(data: &String) -> usize {
    let mut res = 0;
    'rep: for line in data.lines() {
        let levels: Vec<i32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();

        for idx in 0..levels.len() {
            let mut new_report = levels.clone();
            new_report.remove(idx);

            if new_report.iter().tuple_windows()
                .map(|(n1, n2)| n1 - n2)
                .tuple_windows()
                .all(|(d1, d2)| d1 * d2 > 0 && (1..=3).contains(&d1.abs()) && (1..=3).contains(&d2.abs())) {
                    res += 1;
                    continue 'rep;
                }
        }
    }

    println!("Day 2 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    );

    assert_eq!(solve1(&data), 2);
    assert_eq!(solve2(&data), 4);
}