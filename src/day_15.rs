use std::collections::VecDeque;

const K: VecDeque<(&str, usize)> = VecDeque::new();

pub fn solve(input: &str) -> (usize, usize) {
    let mut curr = 0;

    for l in input.split(',') {
        curr += hash(l.as_bytes());
    }

    let mut slots = [K; 256];

    'outer: for instruction in input.split(',') {
        let split_index = instruction
            .find(|c| !char::is_ascii_alphabetic(&c))
            .unwrap();

        let (label, instruction) = instruction.split_at(split_index);

        let slot = hash(label.as_bytes());

        if instruction.chars().next() == Some('-') {
            slots[slot].retain(|&(check_label, _)| check_label != label);
        } else {
            let count = instruction[1..].parse::<usize>().unwrap();

            let (a, b) = slots[slot].as_mut_slices();

            for (check_label, value) in a.iter_mut().chain(b.iter_mut()) {
                if *check_label == label {
                    *check_label = label;
                    *value = count;
                    continue 'outer;
                }
            }

            slots[slot].push_back((label, count));
        }
    }

    let mut sum = 0;

    for (slot_index, slot) in slots.iter().enumerate() {
        for (lense_index, (_, lense_count)) in slot.iter().copied().enumerate() {
            sum += (slot_index + 1) * (lense_index + 1) * lense_count;
        }
    }

    (curr, sum)
}

fn hash(b: &[u8]) -> usize {
    let mut curr = 0;

    for byte in b.iter().copied() {
        curr += byte as usize;
        curr *= 17;
        curr %= 256;
    }

    curr
}
