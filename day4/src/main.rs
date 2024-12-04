use aoclib::AocData;

fn build_grid(a: &AocData) -> Vec<Vec<char>> {
    a.row_chars()
        .expect("rows")
        .map(|row| row.collect())
        .collect()
}

fn search_suffix(grid: &Vec<Vec<char>>, x: usize, y: usize, suffix: &str) -> usize {
    let mut schars = suffix.chars();
    let Some(expected) = schars.next() else {
        panic!("no current char");
    };

    if grid[y][x] != expected {
        panic!(
            "expected {} but got {} at {} {}",
            expected, grid[y][x], x, y
        );
    }

    let Some(search) = schars.next() else {
        // We verified that the remaining character is in the expected place on the grid. No search
        // character means we matched the full suffix.
        return 1;
    };

    let xrange = || {
        if x == 0 {
            // TODO: don't check 0
            [0, 1]
        } else if x == grid[0].len() - 1 {
            [-1, 0]
        } else {
            [-1, 1]
        }
    };

    let yrange = if y == 0 {
        [0, 1]
    } else if y == grid.len() - 1 {
        [-1, 0]
    } else {
        [-1, 1]
    };

    let mut n_found = 0;

    for yy in yrange {
        let ny = (y as isize + yy) as usize;
        for xx in xrange() {
            let nx = (x as isize + xx) as usize;

            if grid[ny][nx] == search {
                n_found += search_suffix(grid, nx, ny, &suffix[1..]);
            }
        }
    }

    n_found
}

fn main() {
    let a = AocData::new("input2.txt").expect("input file");
    let grid = build_grid(&a);

    let mut part1 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &'X' {
                let n = search_suffix(&grid, x, y, "XMAS");
                part1 += n;
                println!("at {},{} found {} XMAS (total {})", x, y, n, part1);
            }
        }
    }

    dbg!(part1);
}
