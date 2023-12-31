pub fn solve(input: &str) -> (usize, usize) {
    let result = part_1(input);

    (result, part_2(input))
}

fn part_1(string: &str) -> usize {
    let boards = string.split("\n\n");

    let mut sum = 0;

    let mut smudges = Vec::new();

    for board in boards {
        let board: Vec<Vec<char>> = board
            .lines()
            .clone()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let row_count = board.len();
        let column_count = board[0].len();

        if let Some(row) = search_dimension_smudge(
            &board,
            row_count,
            column_count,
            |board, column_index, (a, b)| (board[a][column_index], board[b][column_index]),
            0,
        ) {
            sum += row * 100;
            smudges.push(Used::Row(row))
        } else if let Some(column) = search_dimension_smudge(
            &board,
            column_count,
            row_count,
            |board, row_index, (a, b)| (board[row_index][a], board[row_index][b]),
            0,
        ) {
            sum += column;
            smudges.push(Used::Column(column))
        }
    }

    sum
}

fn part_2(string: &str) -> usize {
    let boards = string.split("\n\n");

    let mut sum = 0;

    for board in boards {
        let board: Vec<Vec<char>> = board
            .lines()
            .clone()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let row_count = board.len();
        let column_count = board[0].len();

        if let Some(i) = search_dimension_smudge(
            &board,
            row_count,
            column_count,
            |board, column_index, (a, b)| (board[a][column_index], board[b][column_index]),
            1,
        ) {
            sum += i * 100;
        } else if let Some(i) = search_dimension_smudge(
            &board,
            column_count,
            row_count,
            |board, row_index, (a, b)| (board[row_index][a], board[row_index][b]),
            1,
        ) {
            sum += i;
        }
    }

    sum
}

fn search_dimension_smudge<F: Fn(&Vec<Vec<char>>, usize, (usize, usize)) -> (char, char)>(
    board: &Vec<Vec<char>>,
    dimension_size: usize,
    other_dimension_size: usize,
    access: F,
    smudges: usize,
) -> Option<usize> {
    for dimension in 0..(dimension_size - 1) {
        let reflect_start = dimension + 1;

        let reflect_count = (dimension + 1).min(dimension_size - reflect_start);

        let unmirrored = ((dimension + 1) - reflect_count)..=dimension;
        let mirrored = (reflect_start..(reflect_start + reflect_count)).rev();

        let checking = mirrored.zip(unmirrored);

        let mut diffs = 0;

        for other_dimension in 0..other_dimension_size {
            for check in checking.clone() {
                let (a, b) = access(board, other_dimension, check);
                if a != b {
                    diffs += 1;
                }
            }
        }

        if diffs == smudges {
            return Some(dimension + 1);
        }
    }
    None
}

#[derive(Clone, Copy)]
enum Used {
    Row(usize),
    Column(usize),
}
