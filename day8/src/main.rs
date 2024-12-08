use std::collections::HashSet;

use aoclib::AocData;

pub fn print_map(map: &[Vec<char>]) {
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

pub fn generate_map(a: &AocData) -> Vec<Vec<char>> {
    a.lines()
        .unwrap()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn extract_antenna_locations(a: &AocData) -> impl Iterator<Item = ((isize, isize), char)> + '_ {
    a.lines().unwrap().enumerate().flat_map(|(y, row)| {
        row.chars()
            .enumerate()
            .filter(|(_, col)| col.is_alphanumeric())
            .map(move |(x, col)| ((x as isize, y as isize), col))
    })
}

pub fn generate_antinodes(antennas: &Vec<((isize, isize), char)>) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::new();

    for (pos, freq) in antennas {
        for (pos2, freq2) in antennas {
            if pos == pos2 || freq != freq2 {
                continue;
            }

            let (a1_x, a2_x) = if pos.0 > pos2.0 {
                let xd = pos.0 - pos2.0;
                (pos.0 + xd, pos2.0 - xd)
            } else {
                let xd = pos2.0 - pos.0;
                (pos.0 - xd, pos2.0 + xd)
            };

            let (a1_y, a2_y) = if pos.1 > pos2.1 {
                let yd = pos.1 - pos2.1;
                (pos.1 + yd, pos2.1 - yd)
            } else {
                let yd = pos2.1 - pos.1;
                (pos.1 - yd, pos2.1 + yd)
            };

            antinodes.insert((a1_x, a1_y));
            antinodes.insert((a2_x, a2_y));
        }
    }

    antinodes
}

pub fn inside_bounds(pos: &(isize, isize), bounds: &(usize, usize)) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= bounds.0 as isize || pos.1 >= bounds.1 as isize {
        false
    } else {
        true
    }
}

pub fn generate_antinodes_part2(
    antennas: &Vec<((isize, isize), char)>,
    grid_bounds: (usize, usize),
) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::new();

    for (pos, freq) in antennas {
        for (pos2, freq2) in antennas {
            if pos == pos2 || freq != freq2 {
                continue;
            }

            let mut mul = 1;

            antinodes.insert(*pos);
            antinodes.insert(*pos2);

            loop {
                let (a1_x, a2_x) = if pos.0 > pos2.0 {
                    let xd = (pos.0 - pos2.0) * mul;
                    (pos.0 + xd, pos2.0 - xd)
                } else {
                    let xd = (pos2.0 - pos.0) * mul;
                    (pos.0 - xd, pos2.0 + xd)
                };

                let (a1_y, a2_y) = if pos.1 > pos2.1 {
                    let yd = (pos.1 - pos2.1) * mul;
                    (pos.1 + yd, pos2.1 - yd)
                } else {
                    let yd = (pos2.1 - pos.1) * mul;
                    (pos.1 - yd, pos2.1 + yd)
                };

                let mut any_ins = false;

                let a1 = (a1_x, a1_y);
                let a2 = (a2_x, a2_y);

                if inside_bounds(&a1, &grid_bounds) {
                    antinodes.insert(a1);
                    any_ins = true;
                }

                if inside_bounds(&a2, &grid_bounds) {
                    antinodes.insert(a2);
                    any_ins = true;
                }

                if any_ins {
                    mul += 1;
                } else {
                    break;
                }
            }
        }
    }

    antinodes
}

fn main() {
    let a = AocData::new("input.txt").unwrap();
    let mut map = generate_map(&a);
    let antennas: Vec<((isize, isize), char)> = extract_antenna_locations(&a).collect();
    let antinodes: Vec<_> = generate_antinodes(&antennas)
        .into_iter()
        .filter(|(x, y)| {
            x >= &0 && *x < map[0].len() as isize && y >= &0 && *y < map.len() as isize
        })
        .collect();

    let part1 = antinodes.len();

    let antinodes_pt2: Vec<_> = generate_antinodes_part2(&antennas, (map[0].len(), map.len()))
        .into_iter()
        .filter(|(x, y)| {
            x >= &0 && *x < map[0].len() as isize && y >= &0 && *y < map.len() as isize
        })
        .collect();

    print_map(&map);

    for (x, y) in &antinodes_pt2 {
        map[*y as usize][*x as usize] = '#';
    }

    print_map(&map);

    let part2 = antinodes_pt2.len();

    dbg!(part1, part2);
}
