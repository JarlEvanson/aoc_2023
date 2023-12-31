use fxhash::FxHashSet;

const MIN_SPEED: isize = -1000;
const MAX_SPEED: isize = 1000;

const MIN_POS: isize = 200000000000000;
const MAX_POS: isize = 400000000000000;

pub fn solve(input: &str) -> (usize, usize) {
    let mut hailstones = input
        .lines()
        .map(|line| {
            let (mut pos, mut velocity) = line
                .split_once('@')
                .map(|(a, b)| {
                    (
                        a.split(',').map(|val| val.trim()),
                        b.split(',').map(|val| val.trim()),
                    )
                })
                .unwrap();
            let x = pos.next().unwrap().parse().unwrap();
            let y = pos.next().unwrap().parse().unwrap();
            let z = pos.next().unwrap().parse().unwrap();
            let v_x = velocity.next().unwrap().parse().unwrap();
            let v_y = velocity.next().unwrap().parse().unwrap();
            let v_z = velocity.next().unwrap().parse().unwrap();

            Hailstone {
                x,
                y,
                z,
                v_x,
                v_y,
                v_z,
            }
        })
        .collect::<Vec<_>>();

    let mut part_1 = 0;

    for index_1 in 0..hailstones.len() {
        for index_2 in index_1..hailstones.len() {
            if let Some((x, y)) =
                find_future_intersection_2d(&hailstones[index_1], &hailstones[index_2])
            {
                if MIN_POS <= x && x <= MAX_POS && MIN_POS <= y && y <= MAX_POS {
                    part_1 += 1;
                }
            }
        }
    }

    let mut possible_x_speeds = (MIN_SPEED..=MAX_SPEED).collect::<FxHashSet<_>>();
    let mut possible_y_speeds = (MIN_SPEED..=MAX_SPEED).collect::<FxHashSet<_>>();
    let mut possible_z_speeds = (MIN_SPEED..=MAX_SPEED).collect::<FxHashSet<_>>();

    {
        hailstones.sort_by(|a, b| a.v_x.cmp(&b.v_x));

        let mut iter = hailstones.iter().copied().peekable();

        while let Some(test_hailstone) = iter.next() {
            match iter.peek().copied() {
                Some(hailstone) if test_hailstone.v_x == hailstone.v_x => {
                    let x_diff = (test_hailstone.x - hailstone.x).abs();

                    for failed_value in (MIN_SPEED..=MAX_SPEED).flat_map(|test_speed| {
                        let speed_diff = test_speed - hailstone.v_x;

                        if speed_diff == 0 || x_diff % speed_diff != 0 {
                            Some(test_speed)
                        } else {
                            None
                        }
                    }) {
                        possible_x_speeds.remove(&failed_value);
                    }
                }
                _ => continue,
            }
        }
    }

    {
        hailstones.sort_by(|a, b| a.v_y.cmp(&b.v_y));

        let mut iter = hailstones.iter().copied().peekable();

        while let Some(test_hailstone) = iter.next() {
            match iter.peek().copied() {
                Some(hailstone) if test_hailstone.v_y == hailstone.v_y => {
                    let y_diff = (test_hailstone.y - hailstone.y).abs();

                    for failed_value in (MIN_SPEED..=MAX_SPEED).flat_map(|test_speed| {
                        let speed_diff = test_speed - hailstone.v_y;

                        if speed_diff == 0 || y_diff % speed_diff != 0 {
                            Some(test_speed)
                        } else {
                            None
                        }
                    }) {
                        possible_y_speeds.remove(&failed_value);
                    }
                }
                _ => continue,
            }
        }
    }

    {
        hailstones.sort_by(|a, b| a.v_z.cmp(&b.v_z));

        let mut iter = hailstones.iter().copied().peekable();

        while let Some(test_hailstone) = iter.next() {
            match iter.peek().copied() {
                Some(hailstone) if test_hailstone.v_z == hailstone.v_z => {
                    let z_diff = (test_hailstone.z - hailstone.z).abs();

                    for failed_value in (MIN_SPEED..=MAX_SPEED).flat_map(|test_speed| {
                        let speed_diff = test_speed - hailstone.v_z;

                        if speed_diff == 0 || z_diff % speed_diff != 0 {
                            Some(test_speed)
                        } else {
                            None
                        }
                    }) {
                        possible_z_speeds.remove(&failed_value);
                    }
                }
                _ => continue,
            }
        }
    }

    let v_x = possible_x_speeds.into_iter().next().unwrap();
    let v_y = possible_y_speeds.into_iter().next().unwrap();
    let v_z = possible_z_speeds.into_iter().next().unwrap();

    let a = Hailstone {
        x: hailstones[0].x,
        y: hailstones[0].y,
        z: hailstones[0].z,
        v_x: hailstones[0].v_x - v_x,
        v_y: hailstones[0].v_y - v_y,
        v_z: hailstones[0].v_z - v_z,
    };

    let b = Hailstone {
        x: hailstones[1].x,
        y: hailstones[1].y,
        z: hailstones[1].z,
        v_x: hailstones[1].v_x - v_x,
        v_y: hailstones[1].v_y - v_y,
        v_z: hailstones[1].v_z - v_z,
    };

    let Coord { x, y, z } = find_intersection_3d(&a, &b);

    (part_1, (x + y + z) as usize)
}

fn find_future_intersection_2d(a: &Hailstone, b: &Hailstone) -> Option<(isize, isize)> {
    let ma = a.v_y as f64 / a.v_x as f64;
    let mb = b.v_y as f64 / b.v_x as f64;

    if ma == mb {
        return None;
    }

    let ca = a.y as f64 - (ma * a.x as f64);
    let cb = b.y as f64 - (mb * b.x as f64);

    let x_pos = ((cb - ca) / (ma - mb)).round();
    let y_pos = (ma * x_pos + ca).round();

    if ((x_pos as isize) < a.x && a.v_x > 0)
        || ((x_pos as isize) > a.x && a.v_x < 0)
        || ((x_pos as isize) < b.x && b.v_x > 0)
        || ((x_pos as isize) > b.x && b.v_x < 0)
    {
        return None;
    }

    Some((x_pos as isize, y_pos as isize))
}

fn find_intersection_3d(a: &Hailstone, b: &Hailstone) -> Coord {
    let ma = a.v_y as f64 / a.v_x as f64;
    let mb = b.v_y as f64 / b.v_x as f64;

    let ca = a.y as f64 - (ma * a.x as f64);
    let cb = b.y as f64 - (mb * b.x as f64);

    let x_pos = ((cb - ca) / (ma - mb)).round();
    let y_pos = (ma * x_pos + ca).round();

    let time = (x_pos - a.x as f64) / a.v_x as f64;
    let z_pos = (a.z as f64 + (a.v_z as f64) * time).round();

    Coord {
        x: x_pos as isize,
        y: y_pos as isize,
        z: z_pos as isize,
    }
}

#[derive(Clone, Copy, Debug)]
struct Hailstone {
    x: isize,
    y: isize,
    z: isize,
    v_x: isize,
    v_y: isize,
    v_z: isize,
}

struct Coord {
    x: isize,
    y: isize,
    z: isize,
}
