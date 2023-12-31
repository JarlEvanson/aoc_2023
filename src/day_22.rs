use std::collections::VecDeque;

use fxhash::FxHashSet;

pub fn solve(input: &str) -> (usize, usize) {
    let mut bricks = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (mut coord_1, mut coord_2) = line
                .split_once('~')
                .map(|(coord_1, coord_2)| (coord_1.split(','), coord_2.split(',')))
                .unwrap();

            let x_1 = coord_1.next().unwrap().parse::<usize>().unwrap();
            let y_1 = coord_1.next().unwrap().parse::<usize>().unwrap();
            let z_1 = coord_1.next().unwrap().parse::<usize>().unwrap();

            let x_2 = coord_2.next().unwrap().parse::<usize>().unwrap();
            let y_2 = coord_2.next().unwrap().parse::<usize>().unwrap();
            let z_2 = coord_2.next().unwrap().parse::<usize>().unwrap();

            Brick {
                id,
                end_1: Coords {
                    x: x_1,
                    y: y_1,
                    z: z_1,
                },
                end_2: Coords {
                    x: x_2,
                    y: y_2,
                    z: z_2,
                },
            }
        })
        .collect::<Vec<_>>();

    bricks.sort_unstable_by(|brick_1, brick_2| {
        brick_1
            .end_1
            .z
            .min(brick_1.end_2.z)
            .cmp(&brick_2.end_1.z.min(brick_2.end_2.z))
    });

    let mut grid_width = usize::MIN;
    let mut grid_depth = usize::MIN;
    let mut grid_height = usize::MIN;

    for brick in bricks.iter() {
        grid_width = grid_width.max(brick.end_1.x.max(brick.end_2.x));

        grid_depth = grid_depth.max(brick.end_1.y.max(brick.end_2.y));

        grid_height = grid_height.max(brick.end_1.z.max(brick.end_2.z));
    }

    let grid_width = grid_width + 1;
    let grid_depth = grid_depth + 1;
    let grid_height = grid_height + 1;

    let mut grid = vec![vec![vec![usize::MAX; grid_width]; grid_depth]; grid_height];

    for brick_index in 0..bricks.len() {
        let brick = bricks[brick_index];

        let max_x = brick.end_1.x.max(brick.end_2.x);
        let max_y = brick.end_1.y.max(brick.end_2.y);
        let mut max_z = brick.end_1.z.max(brick.end_2.z);

        let min_x = brick.end_1.x.min(brick.end_2.x);
        let min_y = brick.end_1.y.min(brick.end_2.y);
        let mut min_z = brick.end_1.z.min(brick.end_2.z);

        'dropping: while min_z > 0 {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if grid[min_z - 1][y][x] != usize::MAX {
                        break 'dropping;
                    }
                }
            }

            min_z -= 1;
            max_z -= 1;
        }

        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    grid[z as usize][y as usize][x as usize] = brick.id;
                }
            }
        }
    }

    let mut supported_by = vec![FxHashSet::default(); bricks.len()];

    let mut supports = vec![FxHashSet::default(); bricks.len()];

    for z in (0..grid_height - 1).rev() {
        for y in 0..grid_depth as usize {
            for x in 0..grid_width {
                let supported = grid[z + 1][y][x];
                let supporter = grid[z][y][x];

                if supported != supporter && supported != usize::MAX && supporter != usize::MAX {
                    supported_by[supported].insert(supporter);

                    supports[supporter].insert(supported);
                }
            }
        }
    }

    let brick_min_supported = (0..bricks.len())
        .map(|brick_id| {
            supported_by
                .iter()
                .filter(|supporters| supporters.contains(&brick_id))
                .fold(usize::MAX, |state, map| state.min(map.len()))
        })
        .collect::<Vec<_>>();

    let part_1 = brick_min_supported
        .iter()
        .filter(|&&supportee_count| supportee_count != 1)
        .count();

    let mut part_2 = 0;

    for brick_index in brick_min_supported
        .iter()
        .enumerate()
        .filter(|&(_, supportee_count)| *supportee_count == 1)
        .map(|(index, _)| index)
    {
        let mut has_moved = FxHashSet::default();
        let mut unchecked = VecDeque::new();

        unchecked.push_back(brick_index);

        while let Some(fallen) = unchecked.pop_front() {
            if has_moved.contains(&fallen) {
                continue;
            }
            has_moved.insert(fallen);

            for potential_drop in supports[fallen]
                .iter()
                .filter(|&&supported| {
                    supported_by[supported]
                        .iter()
                        .all(|supporter| has_moved.contains(supporter))
                })
                .copied()
            {
                unchecked.push_back(potential_drop);
            }
        }

        part_2 += has_moved.len() - 1;
    }

    (part_1, part_2)
}

#[derive(Clone, Copy, Debug)]
struct Brick {
    id: usize,
    end_1: Coords,
    end_2: Coords,
}

#[derive(Clone, Copy, Debug)]
struct Coords {
    x: usize,
    y: usize,
    z: usize,
}
