#![allow(dead_code)]
use std::fmt;

use rand::seq::SliceRandom;
use rand::thread_rng;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumString)]
enum Suit {
    Man,
    Pin,
    Sou,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumString)]
enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumString)]
enum Wind {
    East,
    South,
    West,
    North,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumString)]
enum Dragon {
    White,
    Green,
    Red,
}

#[derive(Clone, Debug, Ord, PartialEq, Eq, PartialOrd)]
enum Tile {
    Number(Suit, Rank),
    Wind(Wind),
    Dragon(Dragon),
}

impl Tile {
    fn is_honor(&self) -> bool {
        matches!(self, Tile::Dragon(_)) || matches!(self, Tile::Wind(_))
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Number(suit, rank) => write!(f, "{:?} {:?}", rank, suit),
            Tile::Wind(wind) => write!(f, "{:?} Wind", wind),
            Tile::Dragon(dragon) => write!(f, "{:?} Dragon", dragon),
        }
    }
}

#[derive(Debug)]
struct Hand {
    tiles: Vec<Tile>,
    tenpai: bool,
    shanten: usize,
}

impl Hand {
    fn new(initial_tiles: Vec<Tile>) -> Self {
        Hand {
            tiles: initial_tiles,
            tenpai: false,
            shanten: 0,
        }
    }

    fn sort(&mut self) {
        self.tiles.sort()
    }

    fn add_card(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    fn is_yakuman(&self) -> bool {
        true
    }

    //needs to read board state as well (maybe should be a board function)
    fn score(&self) -> i32 {
        if self.is_yakuman() {
            return 8000;
        }

        0
    }

    fn score_old(&mut self) -> i32 {
        let mut jun_count = 0;
        let mut kou_count = 0;
        let mut yaku_count = 0;

        self.sort();

        for i in 0..(self.tiles.len() - 2) {
            let tile1 = &self.tiles[i];
            let tile2 = &self.tiles[i + 1];
            let tile3 = &self.tiles[i + 2];

            if (tile1 == tile2) && (tile2 == tile3) {
                kou_count += 1;

                if matches!(tile1, Tile::Dragon(_)) {
                    yaku_count += 1;
                }
            }
        }

        if kou_count == 4 || jun_count == 4 {
            yaku_count += 1;
        }

        0
    }
}

enum HandState {}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "- Hand -")?;
        for tile in &self.tiles {
            writeln!(f, "{}", tile)?
        }
        Ok(())
    }
}

struct Wall {
    tiles: Vec<Tile>,
}

impl Wall {
    fn new() -> Self {
        let mut wall = Wall { tiles: Vec::new() };
        wall.initialize();

        wall
    }

    fn size(&self) -> usize {
        self.tiles.len()
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.tiles.shuffle(&mut rng);
    }

    fn pop(&mut self) -> Option<Tile> {
        self.tiles.pop()
    }

    fn draw_hand(&mut self) -> Option<Vec<Tile>> {
        let hand_size = 13;
        let mut hand: Vec<Tile> = Vec::new();

        for _ in 0..hand_size {
            match self.pop() {
                Some(value) => hand.push(value),
                None => return None,
            }
        }

        Some(hand)
    }

    fn initialize(&mut self) {
        let num_tiles: i32 = 4;

        for suit in Suit::iter() {
            for rank in Rank::iter() {
                for _ in 0..num_tiles {
                    self.tiles.push(Tile::Number(suit.clone(), rank.clone()));
                }
            }
        }

        for wind in Wind::iter() {
            for _ in 0..num_tiles {
                self.tiles.push(Tile::Wind(wind.clone()));
            }
        }

        for dragon in Dragon::iter() {
            for _ in 0..num_tiles {
                self.tiles.push(Tile::Dragon(dragon.clone()));
            }
        }
    }
}

fn main() {
    let mut wall = Wall::new();
    wall.shuffle();

    let mut p1_hand = Hand::new(wall.draw_hand().unwrap());

    println!("{}", p1_hand);

    p1_hand.sort();

    println!("{}", p1_hand);
}
