use std::collections::HashMap;

pub fn solve1(data: &str) -> usize {
    let mut stones: Vec<String> = data.split_whitespace().map(|s| s.to_string()).collect();
    // println!("{stones:?}");

    for _ in 0..25 {
        stones = stones.iter().flat_map(|stone| {
            match stone {
                _ if stone == &"0".to_string() => vec![String::from("1")],
                _ if stone.len() % 2 == 0 => {
                    let (left, right) = stone.split_at(stone.len() / 2);
                    vec![left.parse::<u128>().unwrap().to_string(), right.parse::<u128>().unwrap().to_string()]
                }
                _ => vec![(stone.parse::<u128>().unwrap() * 2024).to_string()]
            }
        }).collect();
        // println!("{stones:?}");
    }
    let res = stones.len();
    println!("Day 11 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let mut stones: HashMap<u128, usize> = data.split_whitespace().map(|s| (s.parse().unwrap(), 1)).collect();
    // println!("{stones:?}");

    for _ in 0..75 {
        let mut new_stones: HashMap<u128, usize> = HashMap::with_capacity(stones.capacity());
        stones.into_iter().for_each(|(k, v)| {
            match k {
                0 => *new_stones.entry(1).or_insert(0) += v,
                _ if k.to_string().len() % 2 == 0 => {
                    let k = k.to_string();
                    let (left, right) = k.split_at(k.len() / 2);
                    *new_stones.entry(left.parse::<u128>().unwrap()).or_insert(0) += v;
                    *new_stones.entry(right.parse::<u128>().unwrap()).or_insert(0) += v;
                }
                _ => *new_stones.entry(k * 2024).or_insert(0) += v,
            }
        });
        // println!("{new_stones:?}");
        stones = new_stones;
    }

    let res = stones.values().sum();
    println!("Day 11 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"125 17"
    );

    assert_eq!(solve1(&data), 55312);
    // assert_eq!(solve2(&data), 55312);
}