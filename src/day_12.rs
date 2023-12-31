use fxhash::FxHashMap;

pub fn solve(input: &str) -> (usize, usize) {
    (solve_part(input, 1), solve_part(input, 5))
}

fn solve_part(str: &str, reps: usize) -> usize {
    let mut record_vec = Vec::new();
    let mut number_vec = Vec::new();
    let mut map = FxHashMap::default();

    str.lines()
        .map(|line| {
            let (records, numbers) = line.split_once(' ').unwrap();

            record_vec.clear();

            for i in 0..reps {
                record_vec.extend(records.chars());
                if i != reps - 1 {
                    record_vec.push('?');
                }
            }

            number_vec.clear();

            for _ in 0..reps {
                number_vec.extend(numbers.split(',').map(|i| i.parse::<usize>().unwrap()));
            }

            map.clear();

            chain(&mut map, &record_vec, &number_vec, 0, 0, 0)
        })
        .sum::<usize>()
}

/// `skip_count` includes lower state making `skip_count` a valid start pos.
fn chain(
    map: &mut FxHashMap<(usize, usize, usize), usize>,
    dots: &[char],
    blocks: &[usize],
    dot_index: usize,
    block_index: usize,
    broken_count: usize,
) -> usize {
    if let Some(&answer) = map.get(&(dot_index, block_index, broken_count)) {
        return answer;
    }

    if dot_index == dots.len() {
        return if block_index == blocks.len() && broken_count == 0 {
            1
        } else if block_index == blocks.len() - 1 && blocks[block_index] == broken_count {
            1
        } else {
            0
        };
    }

    let mut answer = 0;

    for c in ['.', '#'] {
        if dots[dot_index] == c || dots[dot_index] == '?' {
            if c == '.' && broken_count == 0 {
                answer += chain(map, dots, blocks, dot_index + 1, block_index, 0);
            } else if c == '.'
                && broken_count > 0
                && block_index < blocks.len()
                && blocks[block_index] == broken_count
            {
                answer += chain(map, dots, blocks, dot_index + 1, block_index + 1, 0);
            } else if c == '#' {
                answer += chain(
                    map,
                    dots,
                    blocks,
                    dot_index + 1,
                    block_index,
                    broken_count + 1,
                )
            }
        }
    }

    map.insert((dot_index, block_index, broken_count), answer);

    answer
}
