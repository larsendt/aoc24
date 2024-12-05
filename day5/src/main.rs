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

fn reorder(print_set: &[usize], broken_rule: &(usize, usize)) -> Vec<usize> {
    let Some(a) = print_set.iter().position(|item| &broken_rule.0 == item) else {
        return Vec::from(print_set);
    };

    let Some(b) = print_set.iter().position(|item| &broken_rule.1 == item) else {
        return Vec::from(print_set);
    };

    if a < b {
        return Vec::from(print_set);
    }

    let mut v = Vec::from(&print_set[0..b]);
    v.push(print_set[a]);
    v.push(print_set[b]);
    v.extend(&print_set[b + 1..a]);
    v.extend(&print_set[a + 1..print_set.len()]);

    v
}

fn reorder_all(print_set: &[usize], rules: &[(usize, usize)]) -> Vec<usize> {
    let mut print_set = Vec::from(print_set);
    while !set_ordered(&print_set, rules) {
        for rule in rules {
            print_set = reorder(&print_set, rule);
        }
    }

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

#[cfg(test)]
mod tests {
    use super::{reorder, test_rule};

    #[test]
    fn test_reorder_single_rule() {
        let print_set = [1, 2, 3, 4, 5];
        let rule = (4, 2);
        let reordered = reorder(&print_set, &rule);
        assert_eq!(reordered, [1, 4, 2, 3, 5]);
        assert!(test_rule(&reordered, &rule));
    }

    #[test]
    fn test_reorder_multiple_rule() {
        let print_set = [1, 2, 3, 4, 5];
        let rule1 = (4, 2);
        let rule2 = (5, 1);
        let reordered = reorder(&reorder(&print_set, &rule1), &rule2);

        assert_eq!(reordered, [5, 1, 4, 2, 3]);
        assert!(test_rule(&reordered, &rule1) && test_rule(&reordered, &rule2));
    }
}
