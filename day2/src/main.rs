use itertools::Itertools;
use std::fs::read_to_string;

fn step_is_safe(a: &i32, b: &i32, increasing: bool) -> bool {
    if (increasing && a > b) || (!increasing && a < b) {
        return false;
    }

    let step = (a - b).abs();
    if (1..3).contains(&step) {
        return false;
    }

    true
}

fn is_increasing(line: &&Vec<i32>) -> bool {
    for (a, b) in line.iter().tuple_windows() {
        match a.cmp(b) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => continue,
            std::cmp::Ordering::Greater => false,
        };
    }

    // All elements are the same, or line is empty. Doesn't matter which we choose, it will end
    // up as unsafe anyway except in edge cases??
    false
}

fn clone_remove(line: &[i32], idx_to_remove: usize) -> Vec<i32> {
    let mut line = line.to_vec();
    line.remove(idx_to_remove);
    line
}

fn part2(line: &&Vec<i32>, recurse: bool) -> bool {
    let increasing = is_increasing(line);
    let line = line.to_vec();

    for i in 0..line.len() - 1 {
        let a = line[i];
        let b = line[i + 1];

        if step_is_safe(&a, &b, increasing) {
            continue;
        }

        if !recurse {
            return false;
        }

        // V0 is special because we might have decided increasing or decreasing based on 0->1 (and
        // therefore 0->1 passes), but if we removed 0 the whole set would pass.
        let v0 = clone_remove(&line, 0);
        let v1 = clone_remove(&line, i);
        let v2 = clone_remove(&line, i + 1);

        let v0_pass = part2(&&v0, false);
        let v1_pass = part2(&&v1, false);
        let v2_pass = part2(&&v2, false);

        return v0_pass || v1_pass || v2_pass;
    }

    true
}

fn is_safe_part_2(line: &&Vec<i32>) -> bool {
    part2(line, true)
}

fn is_safe_part_1(line: &&Vec<i32>) -> bool {
    let increasing = is_increasing(line);
    for (a, b) in line.iter().tuple_windows() {
        if !step_is_safe(a, b, increasing) {
            return false;
        }
    }
    true
}

fn main() {
    let levels: Vec<Vec<i32>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    let nsafe_part1 = levels.iter().filter(is_safe_part_1).count();

    let nsafe_part2 = levels.iter().filter(is_safe_part_2).count();

    dbg!(nsafe_part1, nsafe_part2);
}
