use fxhash::FxHashMap;

use crate::util::math::lcm;

pub fn solve(input: &str) -> (usize, usize) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().trim().chars().cycle();

    lines.next();

    let nodes = lines
        .map(|line| line.split_once('=').unwrap())
        .map(|(name, options)| {
            (
                name.trim(),
                options
                    .trim()
                    .trim_end_matches(')')
                    .trim_start_matches('(')
                    .split_once(',')
                    .map(|(l, r)| (l.trim(), r.trim()))
                    .unwrap(),
            )
        })
        .collect::<FxHashMap<&str, (&str, &str)>>();

    let mut current_node = "AAA";

    let mut part_1 = 0;

    for instruction in instructions.clone() {
        if current_node == "ZZZ" {
            break;
        }

        match instruction {
            'L' => {
                current_node = nodes.get(current_node).unwrap().0;
            }
            'R' => {
                current_node = nodes.get(current_node).unwrap().1;
            }
            _ => unimplemented!(),
        }

        part_1 += 1;
    }

    let mut cycle_lengths = Vec::new();

    'k: for node in nodes
        .iter()
        .filter(|&(name, _)| name.ends_with('A'))
        .map(|(name, _)| *name)
    {
        let mut current_node = node;

        let mut steps = 0;

        for instruction in instructions.clone() {
            if current_node.ends_with('Z') {
                cycle_lengths.push(steps);
                continue 'k;
            }

            match instruction {
                'L' => {
                    current_node = nodes.get(current_node).unwrap().0;
                }
                'R' => {
                    current_node = nodes.get(current_node).unwrap().1;
                }
                _ => unimplemented!(),
            }

            steps += 1;
        }
    }

    let part_2 = lcm(&mut cycle_lengths);

    (part_1, part_2)
}
