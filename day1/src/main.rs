use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let (mut first_values, mut second_values): (Vec<usize>, Vec<usize>) =
        read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                (
                    iter.next().unwrap().parse::<usize>().unwrap(),
                    iter.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .unzip();

    first_values.sort();
    second_values.sort();

    let distance: usize = first_values
        .iter()
        .zip(second_values.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum();

    dbg!(distance);

    let mut second_map = HashMap::new();

    for v in second_values {
        *second_map.entry(v).or_insert(0) += 1;
    }

    let simscore: usize = first_values
        .iter()
        .map(|v| second_map.get(v).unwrap_or(&0) * v)
        .sum();
    dbg!(simscore);
}
