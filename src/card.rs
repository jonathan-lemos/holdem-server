use std::slice::Iter;
use std::marker::Copy;
use rand::seq::SliceRandom;
use itertools::Itertools;

/// Represents a hand's rank
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Rank {
    /// Iterates through the ranks in ascending order
    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 13] = [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace];
        RANKS.iter()
    }

    /// Turns a rank into a u32, higher values representing more powerful cards.
    pub fn to_u32(&self) -> u32 {
        match self {
            Rank::Two => 0,
            Rank::Three => 1,
            Rank::Four => 2,
            Rank::Five => 3,
            Rank::Six => 4,
            Rank::Seven => 5,
            Rank::Eight => 6,
            Rank::Nine => 7,
            Rank::Ten => 8,
            Rank::Jack => 9,
            Rank::Queen => 10,
            Rank::King => 11,
            Rank::Ace => 12
        }
    }

    /// Turns a character from Rank::to_char() into a Rank, or None if the char doesn't correspond to a rank
    pub fn from_char(c: char) -> Option<Rank> {
        match c {
            '2' => Some(Rank::Two),
            '3' => Some(Rank::Three),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            '9' => Some(Rank::Nine),
            'T' => Some(Rank::Ten),
            'J' => Some(Rank::Jack),
            'Q' => Some(Rank::Queen),
            'K' => Some(Rank::King),
            'A' => Some(Rank::Ace),
            _ => None
        }
    }

    /// Turns a rank into a unique character
    pub fn to_char(&self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A'
        }
    }

    /// Turns a string from Rank::to_string() into a Rank, or None if the string doesn't correspond to a rank
    pub fn from_string(s: &str) -> Option<Rank> {
        match s {
            "Two" => Some(Rank::Two),
            "Three" => Some(Rank::Three),
            "Four" => Some(Rank::Four),
            "Five" => Some(Rank::Five),
            "Six" => Some(Rank::Six),
            "Seven" => Some(Rank::Seven),
            "Eight" => Some(Rank::Eight),
            "Nine" => Some(Rank::Nine),
            "Ten" => Some(Rank::Ten),
            "Jack" => Some(Rank::Jack),
            "Queen" => Some(Rank::Queen),
            "King" => Some(Rank::King),
            "Ace" => Some(Rank::Ace),
            _ => None
        }
    }

    /// Turns a Rank into a corresponding string
    pub fn to_string(&self) -> &str {
        match self {
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "Jack",
            Rank::Queen => "Queen",
            Rank::King => "King",
            Rank::Ace => "Ace"
        }
    }
}

/// Represents a card's suit
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

impl Suit {
    /// Iterates through the suits
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade];
        SUITS.iter()
    }

    /// Turns a suit into a u32. This is needed for sorting stability.
    pub fn to_u32(&self) -> u32 {
         match self {
            Suit::Diamond => 0,
            Suit::Club => 1,
            Suit::Heart => 2,
            Suit::Spade => 3
        }
    }

    /// Turns a character from Suit::to_char() into a Suit, or None if the char doesn't correspond to a suit
    pub fn from_char(c: char) -> Option<Suit> {
        match c {
            'D' => Some(Suit::Diamond),
            'C' => Some(Suit::Club),
            'H' => Some(Suit::Heart),
            'S' => Some(Suit::Spade),
            _ => None
        }
    }

    /// Turns a suit into its corresponding typeable character
    pub fn to_char(&self) -> char {
        match self {
            Suit::Diamond => 'D',
            Suit::Club => 'C',
            Suit::Heart => 'H',
            Suit::Spade => 'S'
        }
    }
   
    /// Turns a suit into its corresponding symbol.
    pub fn to_symbol(&self) -> char {
        match self {
            Suit::Diamond => '♦',
            Suit::Club => '♣',
            Suit::Heart => '♥',
            Suit::Spade => '♠'
        }
    }

    /// Turns a string from Suit::to_string() into a Suit, or None if the char doesn't correspond to a suit
    pub fn from_string(s: &str) -> Option<Suit> {
        match s {
            "Diamond" => Some(Suit::Diamond),
            "Club" => Some(Suit::Club),
            "Heart" => Some(Suit::Heart),
            "Spade" => Some(Suit::Spade),
            _ => None
        }
    }

    /// Turns a suit into its corresponding string
    fn to_string(&self) -> &str {
        match self {
            Suit::Diamond => "Diamond",
            Suit::Club => "Club",
            Suit::Heart => "Heart",
            Suit::Spade => "Spade"
        }
    }
}

/// Represents a card's suit
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    Set,
    Straight,
    Flush,
    Boat,
    Quads,
    StraightFlush
}

/// Represents a poker card
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    /// Turns a 2-character string into a card. This is meant for quick testing and not actual code.
    pub fn of(s: &str) -> Card {
        if s.len() != 2 {
            panic!("Card string length must be equal to 2");
        }

        let rank = Rank::from_char(s.chars().nth(0).unwrap()).unwrap();
        let suit = Suit::from_char(s.chars().nth(1).unwrap()).unwrap();

        Card {rank: rank, suit: suit}
    }

    pub fn deck() -> [Card; 52] {
        let v: Vec<Card> = Rank::iterator().flat_map(|x| Suit::iterator().map(|y| Card {rank: *x, suit: *y})).collect();
        assert_eq!(v.len(), 52);

        let mut ret: [Card; 52] = [Card { rank: Rank::Two, suit: Suit::Diamond }; 52];
        
        for (e, c) in ret.iter_mut().zip(v) {
            *e = c;
        }

        ret
    }

    pub fn shuffle(c: &mut [Card]) {
        let mut rng = rand::thread_rng();
        c.shuffle(&mut rng);
    }

    pub fn bestFive(cards: &[Card; 7]) -> (HandRank, [Card; 5]) {
        let suits = cards.iter().group_by(|c| c.suit);
    }
}