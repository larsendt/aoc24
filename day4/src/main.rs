use aoclib::AocData;
use std::ops::Range;

fn build_grid(a: &AocData) -> Vec<Vec<char>> {
    a.row_chars()
        .expect("rows")
        .map(|row| row.collect())
        .collect()
}

fn get_cell(
    grid: &[Vec<char>],
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    mul: isize,
) -> Option<char> {
    let dx = mul * dx;
    let dy = mul * dy;
    let xx = x as isize + dx;
    let yy = y as isize + dy;

    if xx >= grid[0].len() as isize || xx < 0 {
        return None;
    }

    if yy >= grid.len() as isize || yy < 0 {
        return None;
    }

    Some(grid[yy as usize][xx as usize])
}

fn search_xmas(grid: &[Vec<char>], x: usize, y: usize) -> usize {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut total = 0;

    for (dx, dy) in deltas {
        let searches = [
            get_cell(grid, x, y, dx, dy, 0),
            get_cell(grid, x, y, dx, dy, 1),
            get_cell(grid, x, y, dx, dy, 2),
            get_cell(grid, x, y, dx, dy, 3),
        ];

        if searches == [Some('X'), Some('M'), Some('A'), Some('S')] {
            total += 1;
        }
    }

    total
}

fn search_x_mas(grid: &[Vec<char>], x: usize, y: usize) -> usize {
    let d1 = [
        get_cell(grid, x, y, 1, 1, -1),
        get_cell(grid, x, y, 1, 1, 0),
        get_cell(grid, x, y, 1, 1, 1),
    ];

    let d2 = [
        get_cell(grid, x, y, -1, 1, -1),
        get_cell(grid, x, y, -1, 1, 0),
        get_cell(grid, x, y, -1, 1, 1),
    ];

    let d1: String = d1.iter().filter_map(|f| *f).collect();
    let d2: String = d2.iter().filter_map(|f| *f).collect();

    if (d1 == "MAS" || d1 == "SAM") && (d2 == "MAS" || d2 == "SAM") {
        1
    } else {
        0
    }
}

fn main() {
    let a = AocData::new("input.txt").expect("input file");
    let grid = build_grid(&a);

    let mut part1 = 0;
    let mut part2 = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'X' {
                part1 += search_xmas(&grid, x, y);
            } else if col == &'A' {
                part2 += search_x_mas(&grid, x, y);
            }
        }
    }

    dbg!(part1, part2);
}
