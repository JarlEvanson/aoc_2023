use fxhash::FxHashMap;

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::grid::{CardinalDirection, Grid, CARDINAL_DIRECTIONS};

pub fn solve(input: &str) -> (usize, usize) {
    let data = input
        .lines()
        .clone()
        .flat_map(|line| line.chars().map(|c| c as u8 - b'0'))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let board = Grid::new(
        data,
        input.lines().next().unwrap().len(),
        input.lines().count(),
    );

    let mut queue = BinaryHeap::new();
    let mut bests = FxHashMap::default();

    for dir in [
        CardinalDirection::North,
        CardinalDirection::South,
        CardinalDirection::West,
        CardinalDirection::East,
    ] {
        queue.push(Reverse(S {
            score: 0,
            state: State {
                row: 0,
                column: 0,
                dir,
                consecutive_moves: 0,
            },
        }));
    }

    while let Some(Reverse(entry)) = queue.pop() {
        if bests.contains_key(&entry.state) {
            continue;
        }

        bests.insert(entry.state, entry.score);

        for neighbor in neighbors(&board, entry.state, 1, 3) {
            let path_from_node =
                entry.score + board.get(neighbor.column, neighbor.row).copied().unwrap() as usize;

            queue.push(Reverse(S {
                score: path_from_node,
                state: neighbor,
            }))
        }
    }

    let part_1 = *bests
        .iter()
        .filter(|&(state, _)| state.row == board.height - 1 && state.column == board.width - 1)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;

    queue.clear();
    bests.clear();

    for dir in [
        CardinalDirection::North,
        CardinalDirection::South,
        CardinalDirection::West,
        CardinalDirection::East,
    ] {
        queue.push(Reverse(S {
            score: 0,
            state: State {
                row: 0,
                column: 0,
                dir,
                consecutive_moves: 0,
            },
        }));
    }

    while let Some(Reverse(entry)) = queue.pop() {
        if bests.contains_key(&entry.state) {
            continue;
        }

        bests.insert(entry.state, entry.score);

        for neighbor in neighbors(&board, entry.state, 4, 10) {
            let path_from_node =
                entry.score + board.get(neighbor.column, neighbor.row).copied().unwrap() as usize;

            queue.push(Reverse(S {
                score: path_from_node,
                state: neighbor,
            }))
        }
    }

    let part_2 = *bests
        .iter()
        .filter(|&(state, _)| state.row == board.height - 1 && state.column == board.width - 1)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;

    (part_1, part_2)
}

fn neighbors(board: &Grid<u8>, state: State, min_moves: usize, max_moves: usize) -> Vec<State> {
    let (x, y) = (state.column as isize, state.row as isize);

    let mut result = Vec::new();

    for dir in CARDINAL_DIRECTIONS {
        let x = x + dir.offset().0;
        let y = y + dir.offset().1;

        if board.get_signed(x, y).is_some() {
            let moves = if dir == state.dir {
                state.consecutive_moves + 1
            } else {
                1
            };

            if dir == state.dir.opposite() {
                continue;
            }

            if moves > max_moves {
                continue;
            }
            if dir != state.dir && state.consecutive_moves < min_moves {
                continue;
            }

            result.push(State {
                row: y as usize,
                column: x as usize,
                dir,
                consecutive_moves: moves,
            });
        }
    }

    result
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct S {
    score: usize,
    state: State,
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    row: usize,
    column: usize,
    dir: CardinalDirection,
    consecutive_moves: usize,
}
