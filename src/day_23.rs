use fxhash::{FxHashMap, FxHashSet};

use crate::util::grid::{CardinalDirection, Grid, CARDINAL_DIRECTIONS};

pub fn solve(input: &str) -> (usize, usize) {
    let data = input
        .lines()
        .clone()
        .flat_map(|line| line.chars())
        .map(|c| c)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let grid = Grid::new(
        data,
        input.lines().next().unwrap().len(),
        input.lines().count(),
    );

    let mut g_1 = FxHashMap::default();
    let mut g_2 = FxHashMap::default();

    for (y, row) in (0..grid.height).map(|row| (row as isize, grid.row(row).unwrap())) {
        for (x, tile) in
            (0..grid.width).map(|column| (column as isize, row.get(column).copied().unwrap()))
        {
            let neighbors: (&[CardinalDirection], &[CardinalDirection]) = match tile {
                '#' => continue,
                '.' => (&CARDINAL_DIRECTIONS, &CARDINAL_DIRECTIONS),
                '^' => (
                    core::slice::from_ref(&CardinalDirection::North),
                    &CARDINAL_DIRECTIONS,
                ),
                '>' => (
                    core::slice::from_ref(&CardinalDirection::East),
                    &CARDINAL_DIRECTIONS,
                ),
                'v' => (
                    core::slice::from_ref(&CardinalDirection::South),
                    &CARDINAL_DIRECTIONS,
                ),
                '<' => (
                    core::slice::from_ref(&CardinalDirection::West),
                    &CARDINAL_DIRECTIONS,
                ),
                _ => unreachable!(),
            };

            let entry_1: &mut Vec<_> = g_1.entry((x, y)).or_default();
            let entry_2: &mut Vec<_> = g_2.entry((x, y)).or_default();

            for (x_offset, y_offset) in neighbors.0.into_iter().map(|dir| dir.offset()) {
                let x = x + x_offset;
                let y = y + y_offset;

                if grid
                    .get_signed(x, y)
                    .copied()
                    .is_some_and(|tile| tile != '#')
                {
                    entry_1.push((x, y, 1));
                }
            }

            for (x_offset, y_offset) in neighbors.1.into_iter().map(|dir| dir.offset()) {
                let x = x + x_offset;
                let y = y + y_offset;

                if grid
                    .get_signed(x, y)
                    .copied()
                    .is_some_and(|tile| tile != '#')
                {
                    entry_2.push((x, y, 1));
                }
            }
        }
    }

    let corridors = g_1
        .iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();

    for (x, y) in corridors {
        let neighbors = g_1.remove(&(x, y)).unwrap();

        let (x_1, y_1, d_1) = neighbors[0];

        let (x_2, y_2, d_2) = neighbors[1];

        if let Some(neighbor) = g_1
            .get_mut(&(x_1, y_1))
            .unwrap()
            .iter_mut()
            .find(|&&mut (test_x, test_y, _)| (test_x, test_y) == (x, y))
        {
            *neighbor = (x_2, y_2, d_1 + d_2);
        }

        if let Some(neighbor) = g_1
            .get_mut(&(x_2, y_2))
            .unwrap()
            .iter_mut()
            .find(|&&mut (test_x, test_y, _)| (test_x, test_y) == (x, y))
        {
            *neighbor = (x_1, y_1, d_1 + d_2);
        }
    }

    let corridors = g_2
        .iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();

    for (x, y) in corridors {
        let neighbors = g_2.remove(&(x, y)).unwrap();

        let (x_1, y_1, d_1) = neighbors[0];

        let (x_2, y_2, d_2) = neighbors[1];

        if let Some(neighbor) = g_2
            .get_mut(&(x_1, y_1))
            .unwrap()
            .iter_mut()
            .find(|&&mut (test_x, test_y, _)| (test_x, test_y) == (x, y))
        {
            *neighbor = (x_2, y_2, d_1 + d_2);
        }

        if let Some(neighbor) = g_2
            .get_mut(&(x_2, y_2))
            .unwrap()
            .iter_mut()
            .find(|&&mut (test_x, test_y, _)| (test_x, test_y) == (x, y))
        {
            *neighbor = (x_1, y_1, d_1 + d_2);
        }
    }

    (
        dfs(
            &g_1,
            grid.height as isize - 1,
            &mut FxHashSet::default(),
            (1, 0),
        )
        .unwrap(),
        dfs(
            &g_2,
            grid.height as isize - 1,
            &mut FxHashSet::default(),
            (1, 0),
        )
        .unwrap(),
    )
}

fn dfs(
    graph: &FxHashMap<(isize, isize), Vec<(isize, isize, usize)>>,
    end_y: isize,
    seen: &mut FxHashSet<(isize, isize)>,
    coords: (isize, isize),
) -> Option<usize> {
    if coords.1 == end_y {
        return Some(0);
    }

    let neighbors = graph.get(&coords).unwrap();

    let mut best = None;

    for (x, y, distance) in neighbors.iter().copied() {
        if seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        let result = dfs(graph, end_y, seen, (x, y));

        if best.is_none()
            || result
                .zip(best)
                .is_some_and(|(result, best)| result + distance > best)
        {
            best = result.map(|total_distance| total_distance + distance);
        }

        seen.remove(&(x, y));
    }

    best
}
