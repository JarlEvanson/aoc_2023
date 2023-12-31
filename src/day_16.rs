use crate::util::grid::CardinalDirection;

const ENEGIZED: u8 = 1 << 4;
const NORTH: u8 = 1 << 3;
const SOUTH: u8 = 1 << 2;
const LEFT: u8 = 1 << 1;
const EAST: u8 = 1 << 0;

pub fn solve(input: &str) -> (usize, usize) {
    let board: Vec<Vec<char>> = input
        .lines()
        .clone()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part_1 = simulate(&board, 0, 0, CardinalDirection::East);

    let top_left = [
        (0, 0, CardinalDirection::East),
        (0, 0, CardinalDirection::South),
    ];
    let top_east = [
        (board[0].len() - 1, 0, CardinalDirection::West),
        (board[0].len() - 1, 0, CardinalDirection::South),
    ];
    let bottom_left = [
        (0, board.len() - 1, CardinalDirection::East),
        (0, board.len() - 1, CardinalDirection::North),
    ];
    let bottom_east = [
        (board[0].len() - 1, board.len() - 1, CardinalDirection::East),
        (
            board[0].len() - 1,
            board.len() - 1,
            CardinalDirection::South,
        ),
    ];

    let iter = top_left
        .into_iter()
        .chain(top_east)
        .chain(bottom_left)
        .chain(bottom_east);
    let iter = iter
        .chain((1..(board[0].len() - 2)).map(|x| (x, 0, CardinalDirection::South)))
        .chain((1..(board[0].len() - 2)).map(|x| (x, board.len() - 1, CardinalDirection::North)));
    let iter = iter
        .chain((1..(board.len() - 2)).map(|y| (0, y, CardinalDirection::East)))
        .chain((1..(board.len() - 2)).map(|y| (board[0].len() - 1, y, CardinalDirection::North)));

    let mut max = 0;

    for (start_x, start_y, start_dir) in iter {
        max = max.max(simulate(&board, start_x, start_y, start_dir));
    }

    (part_1, max)
}

fn simulate(
    board: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    start_dir: CardinalDirection,
) -> usize {
    let mut moves = vec![(start_x, start_y, start_dir)];

    let mut state = vec![vec![0; board[0].len()]; board.len()];

    #[allow(clippy::never_loop)]
    'beam: while let Some((mut x, mut y, mut dir)) = moves.pop() {
        loop {
            let (test_x, test_y, bit) = if x == start_x && y == start_y && state[y][x] == 0 {
                (
                    start_x as isize,
                    start_y as isize,
                    match start_dir {
                        CardinalDirection::North => NORTH,
                        CardinalDirection::South => SOUTH,
                        CardinalDirection::West => LEFT,
                        CardinalDirection::East => EAST,
                    },
                )
            } else {
                match dir {
                    CardinalDirection::North => (x as isize, y as isize - 1, NORTH),
                    CardinalDirection::South => (x as isize, y as isize + 1, SOUTH),
                    CardinalDirection::West => (x as isize - 1, y as isize, LEFT),
                    CardinalDirection::East => (x as isize + 1, y as isize, EAST),
                }
            };

            if test_x < 0
                || test_x >= board[0].len() as isize
                || test_y < 0
                || test_y >= board.len() as isize
            {
                continue 'beam;
            }

            x = test_x as usize;
            y = test_y as usize;

            if state[y][x] & bit == bit {
                continue 'beam;
            }

            state[y][x] |= ENEGIZED | bit;

            match board[y][x] {
                '.' => continue,
                '/' => {
                    dir = match dir {
                        CardinalDirection::North => CardinalDirection::East,
                        CardinalDirection::South => CardinalDirection::West,
                        CardinalDirection::West => CardinalDirection::South,
                        CardinalDirection::East => CardinalDirection::North,
                    }
                }
                '\\' => {
                    dir = match dir {
                        CardinalDirection::North => CardinalDirection::West,
                        CardinalDirection::South => CardinalDirection::East,
                        CardinalDirection::West => CardinalDirection::North,
                        CardinalDirection::East => CardinalDirection::South,
                    }
                }
                '-' if dir == CardinalDirection::West || dir == CardinalDirection::East => {}
                '|' if dir == CardinalDirection::North || dir == CardinalDirection::South => {}
                '-' => {
                    dir = CardinalDirection::West;
                    moves.push((x, y, CardinalDirection::East));
                }
                '|' => {
                    dir = CardinalDirection::North;
                    moves.push((x, y, CardinalDirection::South));
                }
                c => unreachable!("{}", c),
            }
        }
    }

    let mut sum = 0;

    for row in state.iter() {
        for column in row.iter().copied() {
            if column & ENEGIZED == ENEGIZED {
                sum += 1;
            }
        }
    }

    sum
}
