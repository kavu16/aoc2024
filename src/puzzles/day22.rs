use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

use itertools::Itertools;

const PRUNE: isize = 16777216;

pub fn solve1(data: &str) -> isize {
    let res = data
        .lines()
        .flat_map(|secret_seed| {
            successors(secret_seed.parse().ok(), |&secret| {
                Some(secret_hash(secret))
            })
            .nth(2000)
        })
        .sum();

    println!("Day 22 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> isize {
    let mut bananas: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for line in data.lines() {
        let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
        for (p1, p2, p3, p4, p5) in
            successors(line.parse().ok(), |&secret| Some(secret_hash(secret)))
                .take(2000)
                .tuple_windows()
        {
            let (b1, b2, b3, b4, b5) = (p1 % 10, p2 % 10, p3 % 10, p4 % 10, p5 % 10);
            let sequence = (b2 - b1, b3 - b2, b4 - b3, b5 - b4);
            if seen.insert(sequence) {
                *bananas.entry(sequence).or_default() += b5;
            }
        }
    }

    let res = bananas.values().max().unwrap();
    println!("Day 22 Part 2 = {res}");
    *res
}

fn secret_hash(mut secret: isize) -> isize {
    secret ^= secret * 64;
    secret %= PRUNE;

    secret ^= secret / 32;
    secret %= PRUNE;

    secret ^= secret * 2048;
    secret %= PRUNE;
    secret
}

#[test]
fn test_secret_hash() {
    let secrets = successors(Some(123), |&secret| Some(secret_hash(secret)))
        .take(11)
        .collect::<Vec<isize>>();
    assert_eq!(
        secrets,
        vec![
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254
        ]
    )
}

#[test]
fn test_2000th_secret() {
    let secrets: Vec<isize> = vec![1, 10, 100, 2024]
        .into_iter()
        .flat_map(|input| successors(Some(input), |&secret| Some(secret_hash(secret))).nth(2000))
        .collect();

    assert_eq!(secrets, vec![8685429, 4700978, 15273692, 8667524]);
}

#[test]
fn test_sequences() {
    let data = "
1
2
3
2024";
    assert_eq!(solve2(data), 23);
}
