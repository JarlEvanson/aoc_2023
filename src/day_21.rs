use std::collections::VecDeque;

use fxhash::FxHashSet;

use crate::util::grid::Grid;

const STEPS_1: usize = 64;
const STEPS_2: usize = 26501365;

pub fn solve(input: &str) -> (usize, usize) {
    let data = input
        .lines()
        .clone()
        .flat_map(|line| line.chars())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let grid = Grid::new(
        data,
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );

    let mut start_row = 0;
    let mut start_column = 0;

    for row in 0..grid.height as isize {
        for column in 0..grid.width as isize {
            if grid.get_signed(column, row).copied() == Some('S') {
                start_row = row;
                start_column = column;
                break;
            }
        }
    }

    let mut covered = vec![vec![[false, false]; grid.width]; grid.height];

    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_column, 0));
    covered[start_row as usize][start_column as usize][0] = true;

    while let Some((row, column, distance)) = queue.pop_front() {
        let new_distance = distance + 1;

        if distance == STEPS_1 {
            continue;
        }

        for (row_offset, column_offset) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_column = column + column_offset;
            let new_row = row + row_offset;

            if let Some('.' | 'S') = grid.get_signed(new_column, new_row) {
                if !covered[new_row as usize][new_column as usize][new_distance % 2] {
                    covered[new_row as usize][new_column as usize][new_distance % 2] = true;
                    queue.push_back((new_row, new_column, new_distance));
                }
            }
        }
    }

    let part_1 = covered
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&distance| distance[0])
        .count();

    let mut visited = FxHashSet::default();
    let mut current = vec![((start_column, start_row), (0, 0))];
    let mut next = Vec::new();

    let mut d = Vec::with_capacity(3);

    for step in 1..=STEPS_2 {
        visited.clear();

        while let Some(((x, y), (grid_x, grid_y))) = current.pop() {
            if visited.contains(&((x, y), (grid_x, grid_y))) {
                continue;
            }

            visited.insert(((x, y), (grid_x, grid_y)));
            for (row_offset, column_offset) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (new_x, new_grid_x) = {
                    let new_x = x + column_offset;
                    if new_x >= grid.width as isize {
                        (new_x.rem_euclid(grid.width as isize), grid_x + 1)
                    } else if new_x < 0 {
                        (new_x.rem_euclid(grid.width as isize), grid_x - 1)
                    } else {
                        (new_x, grid_x)
                    }
                };

                let (new_y, new_grid_y) = {
                    let new_y = y + row_offset;
                    if new_y >= grid.height as isize {
                        (new_y.rem_euclid(grid.height as isize), grid_y + 1)
                    } else if new_y < 0 {
                        (new_y.rem_euclid(grid.height as isize), grid_y - 1)
                    } else {
                        (new_y, grid_y)
                    }
                };

                if let Some('.' | 'S') = grid.get_signed(new_x, new_y) {
                    next.push(((new_x, new_y), (new_grid_x, new_grid_y)));
                }
            }
        }

        if step % grid.width == STEPS_2 % grid.width {
            next.sort();
            next.dedup();

            d.push(next.len());

            if d.len() == 3 {
                break;
            }
        }

        core::mem::swap(&mut next, &mut current);
    }

    let part_2 = {
        let n = (STEPS_2 / grid.width) as u128;
        let b0 = (d[0]) as u128;
        let b1 = (d[1] - d[0]) as u128;
        let b2 = (d[2] - d[1]) as u128;

        b0 + b1 * n + (n * (n - 1) / 2) * (b2 - b1)
    } as usize;

    (part_1, part_2)
}
