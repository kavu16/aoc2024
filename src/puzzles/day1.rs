use std::collections::{BinaryHeap, HashMap};
pub fn solve1(data: &str) -> u32 {

    let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::new());
    data.lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .for_each(|(l, r)| {
            left.push(l);
            right.push(r);
        });
        
    let mut res = 0;
    while let (Some(n1), Some(n2)) = (left.pop(), right.pop()) {
        res += n1.abs_diff(n2);
    }
    println!("Day 1 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u32 {

    let (mut left, mut right) = (Vec::new(), HashMap::new());
    data.lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .for_each(|(l, r)| {
            left.push(l);
            *right.entry(r).or_default() += 1;
        });
        
    let res = left.iter()
        .map(|n| if let Some(r) = right.get(n) { n * r } else { 0 })
        .sum();
    
    println!("Day 1 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"3   4
4   3
2   5
1   3
3   9
3   3"
    );

    assert_eq!(solve1(&data), 11);
    assert_eq!(solve2(&data), 31);
}