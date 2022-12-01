/*
 * Instructions:
 * The AOC runner crate has a few features
 * you can either work with chars or bytes (accept &str or &[u8])
 * or use a generator w/ custom type via #[aoc_generator(dayx, partx)]
 * you can name implementations via #[aoc(dayx, partx, NAME)]
 *
 * That's it. Have fun!
 */

use std::collections::BinaryHeap;

pub struct Elf {
    calories: Vec<u64>,
}

#[aoc_generator(day1, part1)]
#[aoc_generator(day1, part2)]
pub fn generator(input: &str) -> Vec<Elf> {
    let mut vec = Vec::new();
    vec.push(Elf {
        calories: Vec::new(),
    });
    input.lines().fold(vec, |mut acc, e| {
        let acc_len = acc.len();
        assert!(acc_len >= 1);
        if e.len() == 0 {
            acc.push(Elf {
                calories: Vec::new(),
            });
        } else {
            acc.get_mut(acc_len - 1)
                .expect("acc.len > 0")
                .calories
                .push(e.parse().unwrap());
        }
        acc
    })
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> u64 {
    input
        .into_iter()
        .map(|e| e.calories.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> u64 {
    input
        .into_iter()
        .map(|e| e.calories.iter().sum::<u64>())
        .collect::<BinaryHeap<_>>()
        .into_iter_sorted()
        .take(3)
        .sum()
}
