pub fn solve(input: &str) -> (usize, usize) {
    let mut part_1 = 1;

    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap());

    for (time, distance) in times.zip(distances) {
        let mut counter = 0;

        for time_held in 0..=time {
            let speed = time_held;
            let total_distance = speed * (time - time_held);
            if total_distance > distance {
                counter += 1;
            }
        }

        part_1 *= counter;
    }

    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(|ch: char| ch.is_whitespace(), "")
        .parse::<usize>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(|ch: char| ch.is_whitespace(), "")
        .parse::<usize>()
        .unwrap();

    let inner = ((time.pow(2) - 4 * distance) as f64).sqrt();

    let start = ((-(time as f64) + inner) / -2.0).ceil() as usize;
    let end = ((-(time as f64) - inner) / -2.0).floor() as usize;

    (part_1, end - start + 1)
}
