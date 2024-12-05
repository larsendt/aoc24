use std::cmp::Ordering;

use aoclib::AocData;

fn test_rule(print_set: &[usize], rule: &(usize, usize)) -> bool {
    let Some(a) = print_set.iter().position(|item| &rule.0 == item) else {
        return true;
    };

    let Some(b) = print_set.iter().position(|item| &rule.1 == item) else {
        return true;
    };

    a < b
}

fn set_ordered(print_set: &[usize], rules: &[(usize, usize)]) -> bool {
    rules.iter().all(|rule| test_rule(print_set, rule))
}

fn reorder_all(print_set: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let sort_fn = |a: &usize, b: &usize| -> Ordering {
        for rule in rules {
            if rule.0 == *a && rule.1 == *b {
                return Ordering::Less;
            } else if rule.0 == *b && rule.1 == *a {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    };

    let mut print_set = Vec::from(print_set);
    print_set.sort_by(sort_fn);
    print_set
}

fn main() {
    let a = AocData::new("input.txt").unwrap();

    let orderings: Vec<(usize, usize)> = a
        .lines()
        .unwrap()
        .filter(|line| line.contains("|"))
        .map(|line| (line[0..2].parse().unwrap(), line[3..5].parse().unwrap()))
        .collect();

    let print_sets: Vec<Vec<usize>> = a
        .lines()
        .unwrap()
        .filter(|line| line.contains(","))
        .map(|line| line.split(",").map(|page| page.parse().unwrap()).collect())
        .collect();

    let part1: usize = print_sets
        .iter()
        .filter(|p| set_ordered(p, &orderings))
        .map(|p| p.get(p.len() / 2).unwrap())
        .sum();

    let part2_sets: Vec<_> = print_sets
        .iter()
        .filter(|p| !set_ordered(p, &orderings))
        .map(|p| reorder_all(p, &orderings))
        .collect();

    for s in &part2_sets {
        assert!(set_ordered(s, &orderings))
    }

    let part2: usize = part2_sets
        .iter()
        .map(|p| *p.get(p.len() / 2).unwrap())
        .sum();

    dbg!(part1, part2);
}
