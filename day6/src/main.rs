use std::collections::HashSet;

use aoclib::AocData;

fn build_grid(a: &AocData) -> Vec<Vec<char>> {
    a.row_chars()
        .expect("rows")
        .map(|row| row.collect())
        .collect()
}

fn dir_to_vec(dir: char) -> (isize, isize) {
    match dir {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        _ => panic!("unknown dir: {}", dir),
    }
}

fn turn_dir(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("unknown dir: {}", dir),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(Clone, Copy, Debug)]
struct Line {
    start: Pos,
    end: Pos,
}

enum WalkOutcome {
    Stopped(Pos),
    Exited,
}

impl Line {
    pub fn intersection(&self, other: &Line) -> Option<Pos> {
        assert!(
            (self.start.0 == self.end.0 || self.start.1 == self.end.1)
                && (other.start.0 == other.end.0 || other.start.1 == other.end.1)
        );

        if self.start.0 == self.end.0 {
            // start.x = end.x, therefore line is vertical
            // other line's x coordinates must span ours
            if (other.start.0 <= self.start.0 && self.start.0 <= other.end.0)
                || (other.end.0 <= self.start.0 && self.start.0 <= other.start.0)
            {
                // our x coordinate plus other's y coordinate
                Some(Pos(self.start.0, other.start.1))
            } else {
                None
            }
        } else if self.start.1 == self.end.1 {
            //  start.y = end.y therefore line is horizontal
            // other line's y coordinates must span ours
            if (other.start.1 <= self.start.1 && self.start.1 <= other.end.1)
                || (other.end.1 <= self.start.1 && self.start.1 <= other.start.1)
            {
                // Our y coordinate plus the other's x coordinate
                Some(Pos(other.start.0, self.start.1))
            } else {
                None
            }
        } else {
            panic!("line is not grid-aligned")
        }
    }

    fn direction(&self) -> (isize, isize) {
        if self.start.0 == self.end.0 {
            if self.start.1 < self.end.1 {
                // moving down
                (0, 1)
            } else {
                // assume > because yeah
                // moving up
                (0, -1)
            }
        } else if self.start.1 == self.end.1 {
            if self.start.0 < self.end.0 {
                // moving right
                (1, 0)
            } else {
                // assume >
                // moving left
                (-1, 0)
            }
        } else {
            panic!("line is not grid-aligned");
        }
    }

    fn points(&self) -> Vec<Pos> {
        if self.start.0 == self.end.0 {
            if self.start.1 < self.end.1 {
                (self.start.1..self.end.1 + 1)
                    .map(|y| Pos(self.start.0, y))
                    .collect()
            } else {
                (self.end.1..self.start.1 + 1)
                    .rev()
                    .map(|y| Pos(self.start.0, y))
                    .collect()
            }
        } else if self.start.1 == self.end.1 {
            if self.start.0 < self.end.0 {
                (self.start.0..self.end.0 + 1)
                    .map(|x| Pos(x, self.start.1))
                    .collect()
            } else {
                (self.end.0..self.start.0 + 1)
                    .rev()
                    .map(|x| Pos(x, self.start.1))
                    .collect()
            }
        } else {
            panic!()
        }
    }
}

fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    println!();
}

fn walk_line(
    grid: &mut [Vec<char>],
    mut pos: Pos,
    dir: char,
    lines: &mut Vec<Line>,
    mark_path: bool,
) -> WalkOutcome {
    let start_pos = pos;
    loop {
        let dvec = dir_to_vec(dir);
        let x = pos.0 as isize + dvec.0;
        let y = pos.1 as isize + dvec.1;

        if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
            // Mark the previous cell as visited
            if mark_path {
                grid[pos.1][pos.0] = 'X';
            }

            let line = Line {
                start: start_pos,
                end: pos,
            };
            lines.push(line);
            // Step will take us out of the grid, so we exited
            return WalkOutcome::Exited;
        }

        if grid[y as usize][x as usize] == '#' {
            let line = Line {
                start: start_pos,
                end: pos,
            };
            lines.push(line);
            // Step would take us into an obstacle, so stop where we are
            return WalkOutcome::Stopped(pos);
        }

        if mark_path {
            // Take the step
            grid[y as usize][x as usize] = dir;
            // Mark the previous cell as visited
            grid[pos.1][pos.0] = 'X';
        }
        // Update pos
        pos = Pos(x as usize, y as usize);
    }
}

fn locate_initial_pose(grid: &[Vec<char>]) -> (char, Pos) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if "^>v<".contains(*col) {
                return (*col, Pos(x, y));
            }
        }
    }

    panic!("initial pose not found!");
}

fn count_visited(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|col| **col == 'X')
        .count()
}

fn get_intersections(grid: &mut [Vec<char>], path: &[Line]) -> usize {
    let mut count = 0;
    for i in 0..path.len() - 3 {
        // Only interested in intersections between the first and (x*4)th legs of the rectangle(s)
        let first = path[i];
        for j in (i + 3..path.len()).step_by(4) {
            let second = path[j];

            if let Some(intersection) = first.intersection(&second) {
                // the new obstacle needs to be one step along the second line from the intersection point
                let dir = second.direction();
                let ob_x = intersection.0 as isize + dir.0;
                let ob_y = intersection.1 as isize + dir.1;
                grid[ob_y as usize][ob_x as usize] = 'O';
                // print_grid(grid);
                count += 1;
            }
        }
    }

    count
}

fn simulate_obstacle_placement(
    grid: &mut [Vec<char>],
    path: &[Line],
    initial_pos: Pos,
    initial_dir: char,
) -> usize {
    let mut new_obstacles = HashSet::new();

    for line in path {
        for point in line.points() {
            if point == initial_pos {
                // No obstacle at initial position allowed
                continue;
            }

            // Place an obstacle
            grid[point.1][point.0] = '#';

            let mut pos = initial_pos;
            let mut dir = initial_dir;

            // Don't put initial pos in this, it's not "real" <- TODO follow up on this might not be true
            let mut visited_positions = Vec::new();

            let mut looped = false;
            // Start walking. Don't keep the path, we just care whether the guard has stopped at
            // a place they've been before.
            while let WalkOutcome::Stopped(newpos) =
                walk_line(grid, pos, dir, &mut Vec::new(), false)
            {
                // Only decide we've looped if we actually moved on the previous attempt
                // Technically this misses the case where the guard is surrounded on all sides and
                // just turns instead of walking but I don't think that can happen here.
                if !visited_positions.is_empty()
                    && visited_positions[visited_positions.len() - 1] != newpos
                    && visited_positions.contains(&newpos)
                {
                    // Guard stopped in a place they've been before, they will loop now
                    new_obstacles.insert(point);
                    looped = true;
                    break;
                }

                visited_positions.push(newpos);
                pos = newpos;
                dir = turn_dir(dir);
            }

            if looped {
                // Found it! Mark on the map
                grid[point.1][point.0] = 'O';
            } else {
                // Reset the test obstacle
                grid[point.1][point.0] = '.';
            }
        }
    }

    new_obstacles.len()
}

fn main() {
    let a = AocData::new("input.txt").unwrap();
    let mut grid = build_grid(&a);
    let mut g2 = grid.clone();
    let mut g3 = grid.clone();

    print_grid(&grid);

    let (initial_dir, initial_pos) = locate_initial_pose(&grid);
    let mut path = Vec::new();

    let mut dir = initial_dir;
    let mut pos = initial_pos;

    while let WalkOutcome::Stopped(newpos) = walk_line(&mut grid, pos, dir, &mut path, true) {
        pos = newpos;
        dir = turn_dir(dir);
    }

    print_grid(&grid);

    let part1 = count_visited(&grid);

    // Really wanted this one to work but I think something is wrong with the loops that contain >1 rectangle
    let part2 = get_intersections(&mut g2, &path);
    // This one works though
    let other_part2 = simulate_obstacle_placement(&mut g3, &path, initial_pos, initial_dir);

    print_grid(&g2);

    print_grid(&g3);
    dbg!(part1, part2, other_part2);
}
