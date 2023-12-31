pub fn solve(input: &str) -> (usize, usize) {
    let histories: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            {
                line.split_whitespace()
                    .map(|val| val.parse::<isize>().unwrap())
            }
            .collect()
        })
        .collect();

    let mut part_1 = 0;

    for history in histories.clone() {
        let mut rows = vec![history];

        loop {
            let diffs = rows
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0]);

            let diffs = diffs.collect();

            rows.push(diffs);

            if rows.last().unwrap().iter().copied().all(|val| val == 0) {
                break;
            }
        }

        let mut keep = 0;

        for row in (1..rows.len()).rev() {
            keep += rows[row - 1].last().unwrap();
        }

        part_1 += keep;
    }

    let mut part_2 = 0;

    for history in histories {
        let mut rows = vec![history];

        loop {
            let diffs = rows
                .last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0]);

            let diffs = diffs.collect();

            rows.push(diffs);

            if rows.last().unwrap().iter().copied().all(|val| val == 0) {
                break;
            }
        }

        let mut keep = 0;

        for row in (1..rows.len()).rev() {
            keep = rows[row - 1].first().unwrap() - keep;
        }

        part_2 += keep;
    }

    (part_1 as usize, part_2 as usize)
}
