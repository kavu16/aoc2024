use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve1(data: &str) -> u32 {
    let (rules, book) = data.split_once("\n\n").unwrap();
    let (mut before, mut after) = (HashMap::<u32, HashSet<u32>>::new(), HashMap::<u32, HashSet<u32>>::new());

    for rule in rules.lines() {
        let (l, r) = rule.split_once('|').unwrap();
        let (l, r): (u32, u32) = (l.parse().unwrap(), r.parse().unwrap());
        after.entry(l).or_default().insert(r);
        before.entry(r).or_default().insert(l);
    }

    let mut res = 0;
    'p: for pages in book.lines() {
        let pages: Vec<u32> = pages.split(',').map(|p| p.parse().unwrap()).collect();

        let mut seen = HashSet::<u32>::new();
        seen.insert(pages[0]);
        for page in pages.iter().skip(1) {
            if let Some(after_set) = after.get(page) {
                for a in after_set {
                    if seen.contains(a) { continue 'p; }
                }
            }
        }

        seen.clear();
        seen.insert(pages[pages.len() - 1]);
        for page in pages.iter().skip(1) {
            if let Some(before_set) = before.get(page) {
                for b in before_set {
                    if seen.contains(b) { continue 'p; }
                }
            }
        }

        res += pages[pages.len() / 2];
    }
    
    println!("Day 5 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u32 {
    let (rules, book) = data.split_once("\n\n").unwrap();
    let (mut before, mut after) = (HashMap::<u32, HashSet<u32>>::new(), HashMap::<u32, HashSet<u32>>::new());

    for rule in rules.lines() {
        let (l, r) = rule.split_once('|').unwrap();
        let (l, r): (u32, u32) = (l.parse().unwrap(), r.parse().unwrap());
        after.entry(l).or_default().insert(r);
        before.entry(r).or_default().insert(l);
    }

    let mut res = 0;
    'p: for pages in book.lines() {
        let pages: Vec<u32> = pages.split(',').map(|p| p.parse().unwrap()).collect();
        let mut bad_page = false;
        let mut seen = HashSet::<u32>::new();
        seen.insert(pages[0]);
        for page in pages.iter().skip(1) {
            if bad_page { break; }
            if let Some(after_set) = after.get(page) {
                for a in after_set {
                    if seen.contains(a) { bad_page = true; break; }
                }
            }
        }

        seen.clear();
        seen.insert(pages[pages.len() - 1]);
        for page in pages.iter().skip(1) {
            if bad_page { break; }
            if let Some(before_set) = before.get(page) {
                for b in before_set {
                    if seen.contains(b) { bad_page = true; break; }
                }
            }
        }
        if !bad_page { continue 'p; }


        // Topological Sort
        seen.clear();
        let mut corrected = VecDeque::<u32>::new();
        let pages: HashSet<u32> = pages.into_iter().collect();
        fn visit(page: u32, pages: &HashSet<u32>, seen: &mut HashSet<u32>, rules: &HashMap<u32, HashSet<u32>>, corrected: &mut VecDeque<u32>, depth: usize) {
            if seen.contains(&page) || depth == 0 { return; }

            if let Some(after_set) = rules.get(&page) {
                for a in after_set {
                    if !pages.contains(a) { continue; }

                    visit(*a, pages, seen, rules, corrected, depth - 1);
                }
            }
            seen.insert(page);
            corrected.push_front(page);
        }

        for page in pages.iter() {
            visit(*page, &pages, &mut seen, &before, &mut corrected, 20);
        }

        res += corrected[corrected.len() / 2];
    }
    
    println!("Day 5 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    );

    assert_eq!(solve1(&data), 143);
    assert_eq!(solve2(&data), 123);
}