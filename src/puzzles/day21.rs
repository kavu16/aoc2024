use std::{collections::HashMap, iter::once};

use itertools::{iproduct, repeat_n, Itertools};

pub fn solve1(data: &str) -> usize {
    let numpad = HashMap::from([
        ((0, 1), '0'),
        ((0, 2), 'A'),
        ((1, 0), '1'),
        ((1, 1), '2'),
        ((1, 2), '3'),
        ((2, 0), '4'),
        ((2, 1), '5'),
        ((2, 2), '6'),
        ((3, 0), '7'),
        ((3, 1), '8'),
        ((3, 2), '9'),
    ]);

    let arrowpad = HashMap::from([
        ((0, 0), '<'),
        ((0, 1), 'v'),
        ((0, 2), '>'),
        ((1, 1), '^'),
        ((1, 2), 'A'),
    ]);
    let num_opt = optimal_map(&numpad, (0, 0));
    let arrow_opt = optimal_map(&arrowpad, (1, 0));

    let res = data
        .lines()
        .map(|line| {
            let value = line[..line.len() - 1].parse::<usize>().unwrap();
            let robot_1 = optimal_path(format!("A{line}"), &num_opt);
            let robot_2 = optimal_path(format!("A{robot_1}"), &arrow_opt);
            let me = optimal_path(format!("A{robot_2}"), &arrow_opt);
            println!("{line} -> {me}");
            println!("complexity: {}", value * me.len());
            value * me.len()
        })
        .sum();

    println!("Day 21 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> usize {
    let numpad = HashMap::from([
        ((0, 1), '0'),
        ((0, 2), 'A'),
        ((1, 0), '1'),
        ((1, 1), '2'),
        ((1, 2), '3'),
        ((2, 0), '4'),
        ((2, 1), '5'),
        ((2, 2), '6'),
        ((3, 0), '7'),
        ((3, 1), '8'),
        ((3, 2), '9'),
    ]);

    let arrowpad = HashMap::from([
        ((0, 0), '<'),
        ((0, 1), 'v'),
        ((0, 2), '>'),
        ((1, 1), '^'),
        ((1, 2), 'A'),
    ]);
    let num_opt = optimal_map(&numpad, (0, 0));
    let arrow_opt = optimal_map(&arrowpad, (1, 0));

    let res = data
        .lines()
        .map(|line| {
            let value = line[..line.len() - 1].parse::<usize>().unwrap();
            let me = solve_cached(
                line.to_string(),
                26,
                &num_opt,
                &arrow_opt,
                true,
                &mut HashMap::new(),
            );
            value * me
        })
        .sum();

    println!("Day 21 Part 2 = {res}");
    res
}

fn optimal_map(
    keypad: &HashMap<(i32, i32), char>,
    forbidden_index: (i32, i32),
) -> HashMap<(char, char), String> {
    let pairs = iproduct!(keypad.iter(), keypad.iter());
    pairs
        .into_iter()
        .map(|(((x1, y1), &k1), ((x2, y2), &k2))| {
            let mut path = String::default();
            path.extend(repeat_n('<', (y1 - y2).max(0) as usize));
            path.extend(repeat_n('v', (x1 - x2).max(0) as usize));
            path.extend(repeat_n('^', (x2 - x1).max(0) as usize));
            path.extend(repeat_n('>', (y2 - y1).max(0) as usize));
            if (*x1, *y2) == forbidden_index || (*x2, *y1) == forbidden_index {
                path = path.chars().rev().collect();
            }
            path.extend(once('A'));
            ((k1, k2), path)
        })
        .collect()
}

fn optimal_path(input: String, map: &HashMap<(char, char), String>) -> String {
    input
        .chars()
        .tuple_windows()
        .map(|(k1, k2)| map[&(k1, k2)].as_str())
        .collect()
}

fn solve_cached(
    input: String,
    iteration: usize,
    num_opt: &HashMap<(char, char), String>,
    arrow_opt: &HashMap<(char, char), String>,
    first: bool,
    cache: &mut HashMap<(String, usize, bool), usize>,
) -> usize {
    if iteration == 0 {
        return input.len();
    }
    if let Some(&cached) = cache.get(&(input.clone(), iteration, first)) {
        return cached;
    }
    let res = input
        .chars()
        .fold(('A', 0), |(prev, total), c| {
            let graph = if first { num_opt } else { arrow_opt };
            (
                c,
                total
                    + solve_cached(
                        graph[&(prev, c)].clone(),
                        iteration - 1,
                        num_opt,
                        arrow_opt,
                        false,
                        cache,
                    ),
            )
        })
        .1;

    cache.insert((input, iteration, first), res);
    res
}

#[test]
fn test() {
    let data = String::from(
        "029A
980A
179A
456A
379A",
    );
    assert_eq!(solve1(&data), 126384);
}

/*
<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
<v<A>^>A<A>A<AAv>A^A<vAAA^>A
<A^A^^>AvvvA
029A

456A
<<vAA>A^>AA<Av>A^AAvA^A<vA^>A<A>A<vA^>A<A>A<<vA>A^>AA<Av>A^A

*/
