use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve1(data: &str) -> usize {
    let connections: HashMap<&str, HashSet<&str>> = data
        .lines()
        .flat_map(|line| line.split_once('-'))
        .fold(HashMap::new(), |mut connections, (left, right)| {
            connections.entry(left).or_default().insert(right);
            connections.entry(right).or_default().insert(left);
            connections
        });

    let res = connections
        .keys()
        .combinations(3)
        .filter(|triple| {
            triple
                .iter()
                .flat_map(|node| node.chars().collect::<Vec<_>>().first().cloned())
                .any(|c| c == 't')
                && connections.get(triple[0]).unwrap().contains(triple[1])
                && connections.get(triple[1]).unwrap().contains(triple[2])
                && connections.get(triple[2]).unwrap().contains(triple[0])
        })
        .map(|triple| {
            triple
                .iter()
                .sorted_by_key(|node| node.chars().last().unwrap())
                .map(|node| node.chars())
                .flatten()
                .collect::<String>()
        })
        .collect::<HashSet<_>>()
        .len();

    println!("Day 23 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> String {
    let connections: HashMap<String, HashSet<String>> = data
        .lines()
        .flat_map(|line| line.split_once('-'))
        .fold(HashMap::new(), |mut connections, (left, right)| {
            connections
                .entry(left.to_string())
                .or_default()
                .insert(right.to_string());
            connections
                .entry(right.to_string())
                .or_default()
                .insert(left.to_string());
            connections
        });

    let mut cliques = vec![];
    bron_kerbosch(
        HashSet::new(),
        HashSet::from_iter(connections.keys().cloned()),
        HashSet::new(),
        &mut cliques,
        &connections,
    );

    let res = cliques
        .iter()
        .sorted_by_key(|clique| clique.len())
        .last()
        .unwrap()
        .iter()
        .sorted()
        .join(",");

    println!("Day 23 Part 2 = {res}");
    res
}

fn bron_kerbosch(
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
    cliques: &mut Vec<HashSet<String>>,
    connections: &HashMap<String, HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }

    let pivot = p.union(&x).nth(0).unwrap();

    for node in p
        .difference(connections.get(pivot).unwrap())
        .cloned()
        .collect::<HashSet<_>>()
    {
        let mut next_r = r.clone();
        next_r.insert(node.clone());
        bron_kerbosch(
            next_r,
            p.intersection(connections.get(&node).unwrap())
                .cloned()
                .collect(),
            x.intersection(connections.get(&node).unwrap())
                .cloned()
                .collect(),
            cliques,
            connections,
        );
    }
}

#[test]
fn test_example() {
    let data = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    assert_eq!(solve1(data), 7);
    assert_eq!(solve2(data).as_str(), "co,de,ka,ta")
}
