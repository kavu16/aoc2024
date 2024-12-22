use std::collections::{HashMap, HashSet};

fn is_valid(design: String, patterns: &HashSet<&str>) -> bool {
    for i in 0..design.len() {
        if patterns.contains(&design[..i+1]) {
            if patterns.contains(&design[i+1..]) || is_valid(design[i+1..].to_string(), patterns){
                return true;
            }
        }
    }
    false
}

pub fn solve1(data: &str) -> usize {
    let (patterns, designs) = data.split_once("\n\n").unwrap();
    let patterns: HashSet<&str> = patterns.split(", ").collect();

    let designs: Vec<String> = designs.lines().map(|d| d.to_string()).collect();
    
    let res = designs.into_iter().filter(|d| is_valid(d.clone(), &patterns)).count();
    println!("Day 19 Part 1 = {res}");
    res
}

fn valid_count(design: String, patterns: &HashSet<&str>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = cache.get(&design) {
        return count;
    }
    let mut count = 0;
    for i in 0..design.len() {
        if patterns.contains(&design[..i+1]) {
            if patterns.contains(&design[i+1..]) {
                count += 1;
            } 
            count += valid_count(design[i+1..].to_string(), patterns, cache);
        }
    }
    cache.insert(design, count);
    count
}

pub fn solve2(data: &str) -> usize {
    let (patterns, designs) = data.split_once("\n\n").unwrap();
    let patterns: HashSet<&str> = patterns.split(", ").collect();

    let designs: Vec<String> = designs.lines().map(|d| d.to_string()).collect();
    
    let mut cache: HashMap<String, usize> = HashMap::new();
    let res = designs.into_iter().map(|d| valid_count(d.clone(), &patterns, &mut cache)).sum();
    println!("Day 19 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
    );

    assert_eq!(solve1(&data), 6);
    assert_eq!(solve2(&data), 16);
}