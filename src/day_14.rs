use fxhash::FxHashMap;

pub fn solve(input: &str) -> (usize, usize) {
    let mut board: Vec<Vec<char>> = input
        .lines()
        .clone()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut mapping = FxHashMap::default();

    let mut count: usize = 0;

    let mut part_1 = 0;

    while count < 1_000_000_000 {
        if let Some(iter_count) = mapping.get(&board) {
            let diff = count - iter_count;

            while count + diff < 1_000_000_000 {
                count += diff;
            }
        }

        mapping.insert(board.clone(), count);

        for row in 0..board.len() {
            for column in 0..board[row].len() {
                let c = board[row][column];

                if c == 'O' {
                    let mut min_row = row;
                    while min_row > 0
                        && board[min_row - 1][column] != 'O'
                        && board[min_row - 1][column] != '#'
                    {
                        min_row -= 1;
                    }
                    board[row][column] = '.';
                    board[min_row][column] = 'O';
                }
            }
        }

        if count == 0 {
            part_1 = calculate_load(&board);
        }

        for column in 0..board[0].len() {
            for row in 0..board.len() {
                if board[row][column] == 'O' {
                    let mut min_column = column;
                    while min_column > 0
                        && board[row][min_column - 1] != 'O'
                        && board[row][min_column - 1] != '#'
                    {
                        min_column -= 1;
                    }
                    board[row][column] = '.';
                    board[row][min_column] = 'O';
                }
            }
        }

        for row in (0..board.len()).rev() {
            for column in (0..board[row].len()).rev() {
                let c = board[row][column];

                if c == 'O' {
                    let mut max_row = row;
                    while max_row < board.len() - 1
                        && board[max_row + 1][column] != 'O'
                        && board[max_row + 1][column] != '#'
                    {
                        max_row += 1;
                    }
                    board[row][column] = '.';
                    board[max_row][column] = 'O';
                }
            }
        }

        for column in (0..board[0].len()).rev() {
            for row in (0..board.len()).rev() {
                if board[row][column] == 'O' {
                    let mut max_column = column;
                    while max_column < board[0].len() - 1
                        && board[row][max_column + 1] != 'O'
                        && board[row][max_column + 1] != '#'
                    {
                        max_column += 1;
                    }
                    board[row][column] = '.';
                    board[row][max_column] = 'O';
                }
            }
        }

        count += 1;
    }

    (part_1, calculate_load(&board))
}

fn calculate_load(board: &Vec<Vec<char>>) -> usize {
    let mut cost = 0;
    for row in 0..board.len() {
        for column in 0..board[row].len() {
            if board[row][column] == 'O' {
                cost += board.len() - row;
            }
        }
    }
    cost
}
