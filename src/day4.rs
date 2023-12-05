use regex::Regex;
use std::collections::HashSet;

pub static EXAMPLE1: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub fn p1(raw: &str) -> usize {
    raw.lines()
        .map(|ln| {
            let numbers = ln.rsplit(": ").next().unwrap();
            let sets: Vec<&str> = numbers.split(" | ").collect();
            let (winners, owned) = (sets[0], sets[1]);
            let winners: HashSet<usize> = winners
                .split_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect();
            let owned: HashSet<usize> = owned
                .split_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect();

            let same = owned.intersection(&winners).count();
            let score = (0..same).fold(0, |acc, _| if acc == 0 { 1 } else { acc + acc });
            // dbg!(same, score);
            score
        })
        .sum()
}

pub fn p2(raw: &str) -> usize {
    let cardvalues: Vec<usize> = raw
        .lines()
        .map(|ln| {
            let numbers = ln.rsplit(": ").next().unwrap();
            let sets: Vec<&str> = numbers.split(" | ").collect();
            let (winners, owned) = (sets[0], sets[1]);
            let winners: HashSet<usize> = winners
                .split_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect();
            let owned: HashSet<usize> = owned
                .split_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect();

            owned.intersection(&winners).count()
        })
        .collect();

    let mut queue: Vec<usize> = cardvalues
        .iter()
        .enumerate()
        .filter_map(|(i, v)| if v == &0 { None } else { Some(i) })
        .collect();

    let mut total_cards = cardvalues.len();

    loop {
        match queue.pop() {
            None => break,
            Some(cardno) => {
                let m = cardvalues[cardno];
                for x in 1..m + 1 {
                    total_cards += 1;
                    queue.push(cardno + x);
                }
            }
        }
    }

    total_cards
}
