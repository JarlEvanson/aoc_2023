pub fn solve(input: &str) -> (usize, usize) {
    let mut part_1 = 0;

    let mut card_counts = vec![1usize; input.lines().count()];

    for (line_index, line) in input.lines().enumerate() {
        let (winning, numbers) = line.split_once(':').unwrap().1.split_once('|').unwrap();

        let mut winning_numbers = [0; 10];

        for (index, winning_number) in winning
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .enumerate()
        {
            winning_numbers[index] = winning_number;
        }

        let winning_count = numbers
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .filter(|test| {
                for winning_number in winning_numbers {
                    if *test == winning_number {
                        return true;
                    }
                }
                false
            })
            .count();

        if winning_count != 0 {
            part_1 += 2usize.pow(winning_count as u32 - 1);
        } else {
            part_1 += 0;
        }

        let card_current_count = card_counts[line_index];

        for index in 0..winning_count {
            if let Some(count) = card_counts.get_mut(line_index + index + 1) {
                *count += card_current_count;
            }
        }
    }

    (part_1, card_counts.into_iter().sum::<usize>())
}
