use std::str::FromStr;

pub fn solve(input: &str) -> (usize, usize) {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for (id, line) in input.lines().enumerate() {
        let game = Game::from_str(line).unwrap();

        if game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14 {
            part_1 += id + 1;
        }

        part_2 += game.max_blue * game.max_green * game.max_red;
    }

    (part_1, part_2)
}

#[derive(Debug, Default)]
struct Game {
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.split_once(": ").unwrap().1;

        let mut s = Game {
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        for item in string.split([',', ';']) {
            let mut iter = item.trim_start().split_ascii_whitespace();

            let count = iter.next().unwrap().parse::<usize>().unwrap();
            match iter.next().unwrap() {
                "green" => {
                    s.max_green = s.max_green.max(count);
                }
                "blue" => {
                    s.max_blue = s.max_blue.max(count);
                }
                "red" => {
                    s.max_red = s.max_red.max(count);
                }
                err => unimplemented!("{err}"),
            };
        }

        Ok(s)
    }
}
