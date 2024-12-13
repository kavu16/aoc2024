use std::collections::{BinaryHeap, VecDeque};

pub fn solve1(data: &str) -> u128 {
    let mut free_space = 0;
    let disk: Vec<Option<u128>> = data.char_indices().map(|(idx, c)| {
        if idx % 2 == 0 {
            vec![Some(idx as u128 / 2); c.to_digit(10).unwrap() as usize]
        } else {
            let free = c.to_digit(10).unwrap() as usize;
            free_space += free;
            vec![None; free]
        }
    }).flatten().collect();
    let n = disk.len();

    let mut filler = disk.iter().filter(|&c| c.is_some()).rev();

    let compact: Vec<u128> = disk.iter().enumerate().filter_map(|(idx, c)| {
        match (idx, c) {
            (i, None) if i < n - free_space => {
                *filler.next().unwrap()
            }
            (i, _) if i >= n - free_space => None,
            (_, &c) => c,
        }
    }).collect();

    let res = compact.into_iter().enumerate().map(|(idx, n)| {
        idx as u128 * n
    }).sum();

    println!("Day 9 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> u128 {
    let mut group_idx = 0;
    let mut free_space: VecDeque<(u128, usize)> = VecDeque::new();
    let mut groups: VecDeque<(Vec<Option<u128>>, usize)> = VecDeque::new();

    let mut disk: Vec<Option<u128>> = data.char_indices().map(|(idx, c)| {
        if idx % 2 == 0 {
            let id = idx as u128 / 2;
            let len = c.to_digit(10).unwrap() as usize;
            groups.push_back((vec![Some(id); len], group_idx));
            group_idx += len;
            vec![Some(id); len]
        } else {
            let free = c.to_digit(10).unwrap() as usize;
            if free != 0 {free_space.push_back((free as u128, group_idx));}
            group_idx += free;
            vec![None; free]
        }
    }).flatten().collect();

    while let Some((group, group_idx)) = groups.pop_back() {
        let mut used: VecDeque<(u128, usize)> = VecDeque::new();
        while let Some((free, free_idx)) = free_space.pop_front() {
            if group.len() as u128 <= free {
                disk.splice(group_idx..group_idx + group.len(), vec![None; group.len()].into_iter());
                if free - group.len() as u128 > 0 {
                    free_space.push_front((free-group.len() as u128, free_idx + free as usize - group.len() + 1));
                }
                disk.splice(free_idx..free_idx + group.len(), group.into_iter());
                // println!("{}", disk.iter().map(|n| {if let Some(n) = n { n.to_string() } else { ".".to_owned() }}).collect::<String>());
                // println!("{free_space:?}");
                break;
            } else {
                used.push_back((free, free_idx));
            }
        }
        for u in used.into_iter().rev() {
            free_space.push_front(u);
        }
    }

    let res = disk.into_iter().enumerate().map(|(idx, val)| {
        if let Some(n) = val {
            idx as u128 * n
        } else {
            0
        }
    }).sum();

    println!("Day 9 Part 1 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"2333133121414131402"
    );

    assert_eq!(solve1(&data), 1928);
    assert_eq!(solve2(&data), 2858);
}