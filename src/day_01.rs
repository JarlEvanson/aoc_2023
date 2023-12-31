pub fn solve(input: &str) -> (usize, usize) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in input.lines() {
        let iter = line.bytes().filter(|c| c.is_ascii_digit());
        part_1 +=
            ((iter.clone().next().unwrap() - b'0') * 10 + (iter.last().unwrap() - b'0')) as usize;

        let mut first = None;
        let mut last = 0;

        for start in 0..line.len() {
            let test = &line[start..];

            if test.starts_with('1') || test.starts_with("one") {
                first.get_or_insert(1);
                last = 1;
            } else if test.starts_with('2') || test.starts_with("two") {
                first.get_or_insert(2);
                last = 2;
            } else if test.starts_with('3') || test.starts_with("three") {
                first.get_or_insert(3);
                last = 3;
            } else if test.starts_with('4') || test.starts_with("four") {
                first.get_or_insert(4);
                last = 4;
            } else if test.starts_with('5') || test.starts_with("five") {
                first.get_or_insert(5);
                last = 5;
            } else if test.starts_with('6') || test.starts_with("six") {
                first.get_or_insert(6);
                last = 6;
            } else if test.starts_with('7') || test.starts_with("seven") {
                first.get_or_insert(7);
                last = 7;
            } else if test.starts_with('8') || test.starts_with("eight") {
                first.get_or_insert(8);
                last = 8;
            } else if test.starts_with('9') || test.starts_with("nine") {
                first.get_or_insert(9);
                last = 9;
            }
        }

        part_2 += unsafe { first.unwrap_unchecked() * 10 + last };
    }

    (part_1, part_2)
}
