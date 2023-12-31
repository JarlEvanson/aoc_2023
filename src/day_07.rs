use std::cmp::Ordering;

pub fn solve(input: &str) -> (usize, usize) {
    let mut part_1 = Vec::new();
    let mut part_2 = Vec::new();

    for (hand, bid) in input.lines().map(|val| val.split_once(' ').unwrap()) {
        let bid = bid.parse::<usize>().unwrap();

        let mut hand = hand.chars().map(|ch| match ch {
            'A' => 12u8,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            ch => unimplemented!("{}", ch),
        });

        let a = hand.next().unwrap();
        let b = hand.next().unwrap();
        let c = hand.next().unwrap();
        let d = hand.next().unwrap();
        let e = hand.next().unwrap();

        let mut hand = [a, b, c, d, e];

        let mut card_counts = [0u8; 13];

        for card in hand.iter().copied() {
            card_counts[card as usize] += 1;
        }

        part_1.push((hand, bid, classify_hand(card_counts)));

        let joker_count = core::mem::take(&mut card_counts[9]);

        *card_counts.iter_mut().max().unwrap() += joker_count;

        for card in hand.iter_mut() {
            match card.cmp(&&mut 9) {
                Ordering::Equal => *card = 0,
                _ => *card += 1,
            }
        }

        part_2.push((hand, bid, classify_hand(card_counts)))
    }

    part_1.sort_unstable_by(|(hand1, _, kind1), (hand2, _, kind2)| {
        if !kind1.eq(kind2) {
            return kind1.cmp(kind2);
        }

        for (card1, card2) in hand1.iter().copied().zip(hand2.iter().copied()) {
            if card1 != card2 {
                return card1.cmp(&card2);
            }
        }

        std::cmp::Ordering::Equal
    });

    let part_1 = part_1
        .iter()
        .enumerate()
        .map(|(index, (_, bid, _))| (index + 1) * bid)
        .sum::<usize>();

    part_2.sort_unstable_by(|(hand1, _, kind1), (hand2, _, kind2)| {
        if !kind1.eq(kind2) {
            return kind1.cmp(kind2);
        }

        for (card1, card2) in hand1.iter().copied().zip(hand2.iter().copied()) {
            if card1 != card2 {
                return card1.cmp(&card2);
            }
        }

        std::cmp::Ordering::Equal
    });

    let part_2 = part_2
        .iter()
        .enumerate()
        .map(|(index, (_, bid, _))| (index + 1) * bid)
        .sum::<usize>();

    (part_1, part_2)
}

fn classify_hand(card_counts: [u8; 13]) -> HandType {
    if card_counts.iter().copied().any(|count| count == 5) {
        HandType::Five
    } else if card_counts.iter().copied().any(|count| count == 4) {
        HandType::Four
    } else if card_counts.iter().copied().any(|count| count == 3)
        && card_counts.iter().copied().any(|count| count == 2)
    {
        HandType::Full
    } else if card_counts.iter().copied().any(|count| count == 3) {
        HandType::Three
    } else if card_counts
        .iter()
        .copied()
        .filter(|&count| count == 2)
        .count()
        == 2
    {
        HandType::Two
    } else if card_counts.iter().copied().any(|count| count == 2) {
        HandType::One
    } else {
        HandType::High
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum HandType {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}
