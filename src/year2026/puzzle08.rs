use std::collections::HashMap;
use std::hash::Hash;

type Stoats<K> = HashMap<K, u64>;
type Rules<K> = HashMap<K, Stoats<K>>;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    evolve(&single_rules(input), HashMap::from([("A", 1), ("B", 1)]), 7)
}

pub fn part2(input: &str) -> u64 {
    evolve(&double_rules(input), HashMap::from([(("A", "B"), 1)]), 7) + 1
}

pub fn part3(input: &str) -> u64 {
    evolve(&double_rules(input), HashMap::from([(("A", "B"), 1)]), 21) + 1
}

fn single_rules(input: &str) -> Rules<&str> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let key = tokens.next().unwrap();
        rules.entry(key).or_insert_with(|| frequencies(tokens));
    }

    rules
}

fn double_rules(input: &str) -> Rules<(&str, &str)> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let mut tokens: Vec<_> = line.split_ascii_whitespace().collect();
        tokens[1..].rotate_left(1);
        let last = tokens.len() - 1;

        for _ in 0..2 {
            let key = (tokens[0], tokens[last]);
            let pairs = tokens.array_windows().map(|&[a, b]| (a, b));
            rules.entry(key).or_insert_with(|| frequencies(pairs));
            tokens.swap(0, last);
        }
    }

    rules
}

fn frequencies<K: Eq + Hash>(items: impl Iterator<Item = K>) -> HashMap<K, u64> {
    items.fold(HashMap::new(), |mut count, item| {
        *count.entry(item).or_insert(0) += 1;
        count
    })
}

fn evolve<K>(rules: &Rules<K>, mut current: Stoats<K>, generations: usize) -> u64
where
    K: Copy + Eq + Hash,
{
    for _ in 0..generations {
        let mut next = HashMap::new();

        for (from, amount) in current {
            for (&to, &replacement) in &rules[&from] {
                *next.entry(to).or_insert(0) += amount * replacement;
            }
        }

        current = next;
    }

    current.values().sum()
}
