pub fn solve(input: &str) -> (usize, usize) {
    let instructions = input.lines().map(|line| {
        let mut k = line.split(' ');

        let dir = k.next().unwrap();

        let distance = k.next().unwrap().parse::<usize>().unwrap();

        (dir, distance)
    });

    let mut polygon = vec![(0isize, 0isize)];

    let mut point_count = 0;

    for (dir, distance) in instructions {
        point_count += distance;
        let (x, y) = polygon.last().copied().unwrap();

        let (new_x, new_y) = match dir {
            "R" => (x.checked_add_unsigned(distance).unwrap(), y),
            "L" => (x.checked_sub_unsigned(distance).unwrap(), y),
            "D" => (x, y.checked_add_unsigned(distance).unwrap()),
            "U" => (x, y.checked_sub_unsigned(distance).unwrap()),
            _ => unreachable!(),
        };

        polygon.push((new_x, new_y));
    }

    let part_1 = area(&polygon) + point_count / 2 + 1;

    let instructions = input.lines().map(|line| {
        let k = line
            .split(' ')
            .skip(2)
            .next()
            .unwrap()
            .trim_end_matches(')')
            .trim_start_matches('(');

        let distance = usize::from_str_radix(&k[1..6], 16).unwrap();
        let dir = match k.chars().skip(6).next().unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => unreachable!(),
        };

        (dir, distance)
    });

    let mut polygon = vec![(0isize, 0isize)];

    let mut point_count = 0;

    for (dir, distance) in instructions {
        point_count += distance;
        let (x, y) = polygon.last().copied().unwrap();

        let (new_x, new_y) = match dir {
            "R" => (x.checked_add_unsigned(distance).unwrap(), y),
            "L" => (x.checked_sub_unsigned(distance).unwrap(), y),
            "D" => (x, y.checked_add_unsigned(distance).unwrap()),
            "U" => (x, y.checked_sub_unsigned(distance).unwrap()),
            _ => unreachable!(),
        };

        polygon.push((new_x, new_y));
    }

    let part_2 = area(&polygon) + point_count / 2 + 1;

    (part_1, part_2)
}

fn area(points: &Vec<(isize, isize)>) -> usize {
    let count = points.len();

    let (first_x, first_y) = points[0];
    let (mut prev_x, mut prev_y) = points[0];

    let mut res = 0;

    for i in 1..count {
        let (next_x, next_y) = points[i];

        res += polygon_size(prev_x, prev_y, next_x, next_y);
        (prev_x, prev_y) = (next_x, next_y);
    }

    res += polygon_size(prev_x, prev_y, first_x, first_y);

    (res.abs() / 2).abs() as usize
}

fn polygon_size(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    x1 * y2 - y1 * x2
}
