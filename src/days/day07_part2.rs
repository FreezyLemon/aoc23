use std::cmp::Ordering;

pub struct Day7Part2;

impl crate::days::Day for Day7Part2 {
    fn solve(&self, input: &str) -> String {
        let mut parsed: Vec<_> = input.lines()
            .map(|l| l.split_once(' ').expect("input has ' '"))
            .map(|(hand, bid)| (hand.chars().map(CamelCard::parse).collect::<Vec<_>>(), bid.parse::<i32>().expect("is int")))
            .map(|(cards, bid)| (
                Hand(cards.try_into().expect("has five cards")),
                bid
            ))
            .map(|(hand, bid)| (hand.hand_type(), hand, bid))
            .collect();

        // sorted ascending (weakest card first)
        parsed.sort_by_key(|(_, hand, _)| hand.clone());

        parsed.into_iter()
            .enumerate()
            .map(|(idx, (_, _, bid))| (idx as i32 + 1) * bid)
            .sum::<i32>()
            .to_string()
    }
}

const HAND_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand([CamelCard; HAND_SIZE]);

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut card_counts = Vec::with_capacity(HAND_SIZE);
        let mut joker_count = 0;

        'outer: for c in &self.0 {
            if *c == CamelCard::J {
                joker_count += 1;
                continue;
            }

            for (card, cnt) in &mut card_counts {
                if *card == c {
                    *cnt += 1;
                    continue 'outer;
                }
            }

            card_counts.push((c, 1));
        }

        let mut card_counts: Vec<_> = card_counts.into_iter()
            .map(|(_, cnt)| cnt)
            .collect();

        if joker_count >= 4 {
            return HandType::FiveOfAKind;
        }

        card_counts.sort_unstable();
        *card_counts.last_mut().unwrap() += joker_count;
        match &card_counts[..] {
            [.., 5] => HandType::FiveOfAKind,
            [.., 4] => HandType::FourOfAKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfAKind,
            [.., 2, 2] => HandType::TwoPair,
            [.., 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                // compare cards one by one
                self.0.iter().zip(other.0.iter())
                    .map(|(sc, oc)| sc.cmp(oc))
                    .find(|ord| ord.is_ne())
                    .unwrap_or(Ordering::Equal)
            },
            ordering => ordering,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CamelCard {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    J = 1,
}

impl CamelCard {
    pub fn parse(c: char) -> Self {
        match c {
            'A' => CamelCard::A,
            'K' => CamelCard::K,
            'Q' => CamelCard::Q,
            'J' => CamelCard::J,
            'T' => CamelCard::T,
            '9' => CamelCard::Nine,
            '8' => CamelCard::Eight,
            '7' => CamelCard::Seven,
            '6' => CamelCard::Six,
            '5' => CamelCard::Five,
            '4' => CamelCard::Four,
            '3' => CamelCard::Three,
            '2' => CamelCard::Two,
            _ => panic!("unsupported char {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}
