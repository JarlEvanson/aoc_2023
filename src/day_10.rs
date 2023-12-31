use std::collections::VecDeque;

use crate::util::grid::{CardinalDirection, Grid, CARDINAL_DIRECTIONS};

pub fn solve(input: &str) -> (usize, usize) {
    let mut start_row = 0;
    let mut start_column = 0;

    let data = {
        let mut data = Vec::with_capacity(input.len());

        for (row, line) in input.lines().enumerate() {
            for (column, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Ground,
                    '|' => Tile::NorthSouth,
                    'F' => Tile::SouthEast,
                    'J' => Tile::NorthWest,
                    '7' => Tile::SouthWest,
                    'L' => Tile::NorthEast,
                    '-' => Tile::EastWest,
                    'S' => {
                        start_row = row;
                        start_column = column;
                        Tile::Ground
                    }
                    _ => unreachable!(),
                };

                data.push(tile);
            }
        }

        data.into_boxed_slice()
    };

    let mut grid = Grid::new(
        data,
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );

    let start_row = start_row as isize;
    let start_column = start_column as isize;

    let mut visited_grid = Grid::new(
        vec![false; grid.width * grid.height].into_boxed_slice(),
        grid.width,
        grid.height,
    );

    let mut vec = VecDeque::new();

    {
        let mut directions = Vec::with_capacity(2);

        for direction in CARDINAL_DIRECTIONS {
            let column = start_column + direction.offset().0;
            let row = start_row + direction.offset().1;

            let Some(tile) = grid.get_signed(column, row) else {
                continue;
            };

            let Some(connections) = tile.connects() else {
                continue;
            };

            if connections.contains(&direction.opposite()) {
                directions.push(direction);
            }
        }

        directions.sort();

        for tile in Tile::PIPES {
            if directions.contains(&tile.connects().unwrap()[0])
                && directions.contains(&tile.connects().unwrap()[1])
            {
                *grid.get_mut_signed(start_column, start_row).unwrap() = tile;
            }
        }
    }

    vec.push_back((start_row, start_column, 0));

    let mut part_1 = 0;

    let mut max_row = 0;
    let mut max_column = 0;
    let mut min_row = 0;
    let mut min_column = 0;

    while let Some((row, column, current_distance)) = vec.pop_front() {
        part_1 = part_1.max(current_distance);

        max_row = max_row.max(row);
        max_column = max_column.max(column);
        min_row = min_row.min(row);
        min_column = min_column.min(column);

        for direction in grid
            .get_signed(column, row)
            .unwrap()
            .connects()
            .iter()
            .flat_map(|directions| directions.iter())
        {
            let column = column + direction.offset().0;
            let row = row + direction.offset().1;

            if let Some(visited) = visited_grid.get_signed(column, row).copied() {
                if !visited {
                    *visited_grid.get_mut_signed(column, row).unwrap() = true;
                    vec.push_back((row, column, current_distance + 1));
                }
            }
        }
    }

    let mut part_2 = 0;

    for y in min_row..=max_row {
        let mut parity = false;

        for x in min_column..=max_column {
            let visited = visited_grid.get_signed(x, y).copied().unwrap();
            let tile = grid.get_signed(x, y).copied().unwrap();

            match tile {
                Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast if visited => parity = !parity,
                _ => {}
            }

            if parity && !visited {
                part_2 += 1;
            }
        }
    }

    (part_1, part_2)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

impl Tile {
    const PIPES: [Tile; 6] = [
        Tile::NorthSouth,
        Tile::EastWest,
        Tile::NorthEast,
        Tile::NorthWest,
        Tile::SouthEast,
        Tile::SouthWest,
    ];

    fn connects(&self) -> Option<[CardinalDirection; 2]> {
        match self {
            Tile::Ground => None,
            Tile::NorthSouth => Some([CardinalDirection::North, CardinalDirection::South]),
            Tile::EastWest => Some([CardinalDirection::East, CardinalDirection::West]),
            Tile::NorthEast => Some([CardinalDirection::North, CardinalDirection::East]),
            Tile::NorthWest => Some([CardinalDirection::North, CardinalDirection::West]),
            Tile::SouthEast => Some([CardinalDirection::South, CardinalDirection::East]),
            Tile::SouthWest => Some([CardinalDirection::South, CardinalDirection::West]),
        }
    }
}
