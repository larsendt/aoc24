use std::collections::HashSet;

use aoclib::AocData;

fn make_grid(a: &AocData) -> Vec<Vec<char>> {
    a.input.lines().map(|l| l.chars().collect()).collect()
}

fn get_trail_score(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    search_val: u8,
    trail_prefix: Vec<(usize, usize)>,
) -> (HashSet<Vec<(usize, usize)>>, HashSet<(usize, usize)>) {
    let mut trail_ends = HashSet::new();
    let mut trails = HashSet::new();

    for (xd, yd) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let xx = x as isize + xd;
        let yy = y as isize + yd;

        if xx < 0 || xx as usize >= grid[0].len() || yy < 0 || yy as usize >= grid.len() {
            continue;
        }

        if grid[yy as usize][xx as usize] == char::from_digit(search_val as u32, 10).unwrap() {
            if search_val == 9 {
                trail_ends.insert((xx as usize, yy as usize));
                let mut full_trail = trail_prefix.clone();
                full_trail.push((xx as usize, yy as usize));
                trails.insert(full_trail);
            } else {
                let mut prefix = trail_prefix.clone();
                prefix.push((xx as usize, yy as usize));

                // println!("{} recurse!", " ".repeat(search_val as usize),);
                let (full_trails, ends) =
                    get_trail_score(grid, xx as usize, yy as usize, search_val + 1, prefix);
                trail_ends.extend(ends);
                trails.extend(full_trails);
            }
        }
    }

    (trails, trail_ends)
}

fn score_trails(grid: &[Vec<char>]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '0')
                .map(move |(x, _)| get_trail_score(grid, x, y, 1, vec![(x, y)]))
        })
        .map(|(distinct_trails, trail_ends)| (distinct_trails.len(), trail_ends.len()))
        .fold((0, 0), |acc, item| (acc.0 + item.0, acc.1 + item.1))
}

fn main() {
    let a = AocData::new("input.txt").unwrap();
    let grid = make_grid(&a);
    let part1 = score_trails(&grid);

    dbg!(part1);
}
