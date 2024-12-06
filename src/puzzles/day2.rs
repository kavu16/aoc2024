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
        let diffs = line.split_whitespace().tuple_windows()
            .map(|(l, r)| r.parse::<i32>().unwrap() - l.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        
        // [a b c d e f]
        // [ - + + + +]

        // [5 3 6 7 8]
        // [8 7 6 3 5]
        let n = diffs.len();
        if n <= 1 { res += 1; continue 'rep; }

        let danger = diffs.iter()
            .enumerate()
            .filter(|(idx, &d)| {
                !(1..=3).contains(&(d.abs())) ||
                if *idx > 0 { diffs[idx-1] * d < 0 } else { true } && if *idx < n-1 {diffs[idx + 1] * d < 0 } else { true }
            })
            .collect::<Vec<_>>();

        println!("Diffs = {diffs:?}");
        println!("Potential danger spots: {danger:?}");

        // if danger.len() > 2 { 
        //     // println!("Too many danger spots"); 
        //     continue 'rep; }
        // if danger.len() == 2 && (danger[0].1 == &0 || danger[1].1 == &0 || danger[0].0.abs_diff(danger[1].0) != 1 || !(1..=3).contains(&(danger[0].1 + danger[1].1).abs())) { 
        //     // println!("Faulty combination"); 
        //     continue 'rep; }
        // if danger.len() == 2 && { let new_diff = danger[0].1 + danger[1].1; (if danger[0].0 > 0 { diffs[danger[0].0 - 1] * new_diff < 0 } else { false } || if danger[1].0 < n-1 { diffs[danger[1].0 + 1] * new_diff < 0 } else { false })} { 
        //     // println!("Faulty slopes"); 
        //     continue 'rep; }
        // if danger.len() == 1 && *danger[0].1 != 0 && (1..=n-2).contains(&(danger[0].0)) { 
        //     // println!("Faulty singleton"); 
        //     continue 'rep; }

        res += match danger.len() {
            3.. => {
                eprintln!("Too many dangers");
                0
            }
            2 => {
                let (idx0, idx1) = (danger[0].0, danger[1].0);
                let new_diff = danger[0].1 + danger[1].1;
                if idx0.abs_diff(idx1) != 1 || 
                    !(1..=3).contains(&(new_diff).abs()) || 
                    if idx0 > 0 { diffs[idx0] * new_diff < 0 } else { true } && if idx1 < n-1 {diffs[idx1 + 1] * new_diff < 0 } else { true } {
                        eprintln!("Faulty Pairing");
                        0
                    } else {
                        1
                    }
            }
            1 => {
                if !(1..=n-2).contains(&(danger[0].0)) || danger[0].1 == &0 {
                    1
                } else {
                    let idx = danger[0].0;
                    let diff = danger[0].1;
                    if diff * diffs[idx - 1] > 0 && diff * diffs[idx + 1] > 0 {
                        eprintln!("Same direction");
                        0
                    } else if (1..=3).contains(&(diff + diffs[idx-1]).abs()) || (1..=3).contains(&(diff + diffs[idx+1]).abs()) {
                        1
                    } else {
                        eprintln!("Both too big");
                        0
                    }
                }
            }
            0 => {
                1
            }
        };
        // println!("Marked Safe ^");
        // res += 1;
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