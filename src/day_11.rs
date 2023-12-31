const EXPANSION_CONSTANT: usize = 1_000_000;

pub fn solve(input: &str) -> (usize, usize) {
    let mut galaxies = Vec::new();

    let mut column_expanded = vec![true; input.lines().next().unwrap().len()];
    let mut row_expanded = vec![true; input.lines().count()];

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.bytes().enumerate() {
            if tile == b'#' {
                row_expanded[y] = false;
                column_expanded[x] = false;
                galaxies.push((x, y));
            }
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;

    for index_1 in 0..galaxies.len() {
        for index_2 in (index_1 + 1)..galaxies.len() {
            let (x_1, y_1) = galaxies[index_1];
            let (x_2, y_2) = galaxies[index_2];

            let low_row = y_1.min(y_2);
            let high_row = y_1.max(y_2);

            let low_column = x_1.min(x_2);
            let high_column = x_1.max(x_2);

            for &expanded in row_expanded.iter().take(high_row + 1).skip(low_row + 1) {
                part_1 += expanded as usize + 1;
                part_2 += expanded as usize * (EXPANSION_CONSTANT - 1) + 1;
            }

            for &expanded in column_expanded
                .iter()
                .take(high_column + 1)
                .skip(low_column + 1)
            {
                part_1 += expanded as usize + 1;
                part_2 += expanded as usize * (EXPANSION_CONSTANT - 1) + 1;
            }
        }
    }

    (part_1, part_2)
}
