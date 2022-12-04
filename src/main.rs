#![feature(iter_array_chunks)]

use std::fmt;

pub fn dbg<T: fmt::Debug>(s: &'static str) -> impl Fn(&T) {
    move |x| println!("{s}: {x:?}")
}

pub mod day_1 {
    use std::{io, mem};

    fn inventories() -> impl Iterator<Item = u32> {
        let lines = io::stdin().lines().map(Result::unwrap);
        let meals = lines.map(|line| line.trim().parse::<u32>().ok());
        meals
            .chain([None])
            .scan(0, |inv, meal| match meal {
                Some(meal) => {
                    *inv += meal;
                    Some(None)
                }
                None => Some(Some(mem::take(inv))),
            })
            .flatten()
    }

    pub fn part_1() -> u32 {
        inventories().max().unwrap()
    }

    pub fn part_2() -> u32 {
        inventories()
            .fold([0, 0, 0], |mut acc, inv| {
                let min = *acc.iter().min().unwrap();
                if min < inv {
                    *acc.iter_mut().find(|x| **x == min).unwrap() = inv;
                }
                acc
            })
            .iter()
            .sum()
    }
}

pub mod day_2 {
    use std::{io, str::FromStr};

    use anyhow::Error;

    #[derive(Clone, Copy, Debug)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }

    impl Shape {
        fn score(self) -> u32 {
            match self {
                Self::Rock => 1,
                Self::Paper => 2,
                Self::Scissors => 3,
            }
        }

        fn all() -> impl Iterator<Item = Self> {
            [Self::Rock, Self::Paper, Self::Scissors].into_iter()
        }
    }

    impl FromStr for Shape {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Self::Rock),
                "B" => Ok(Self::Paper),
                "C" => Ok(Self::Scissors),
                _ => panic!("oof"),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum FightOutcome {
        Win,
        Draw,
        Loss,
    }

    impl FightOutcome {
        fn score(self) -> u32 {
            match self {
                Self::Win => 6,
                Self::Draw => 3,
                Self::Loss => 0,
            }
        }
    }

    impl FromStr for FightOutcome {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "X" => Ok(Self::Loss),
                "Y" => Ok(Self::Draw),
                "Z" => Ok(Self::Win),
                _ => panic!("ack"),
            }
        }
    }

    fn fight_outcome(you: Shape, opponent: Shape) -> FightOutcome {
        use Shape::*;

        // TODO: generalize if there are more
        match (you, opponent) {
            (Rock, Scissors) => FightOutcome::Win,
            (Paper, Rock) => FightOutcome::Win,
            (Scissors, Paper) => FightOutcome::Win,

            (Rock, Rock) => FightOutcome::Draw,
            (Paper, Paper) => FightOutcome::Draw,
            (Scissors, Scissors) => FightOutcome::Draw,

            (Rock, Paper) => FightOutcome::Loss,
            (Paper, Scissors) => FightOutcome::Loss,
            (Scissors, Rock) => FightOutcome::Loss,
        }
    }

    fn fights() -> impl Iterator<Item = (Shape, FightOutcome)> {
        io::stdin().lines().flat_map(Result::ok).flat_map(|s| {
            let (a, b) = s.split_once(' ')?;
            Some((a.parse().ok()?, b.parse().ok()?))
        })
    }

    fn score(shape: Shape, outcome: FightOutcome) -> u32 {
        shape.score() + outcome.score()
    }

    pub fn part_1() -> u32 {
        fn misinterpret_as_shape(outcome: FightOutcome) -> Shape {
            match outcome {
                FightOutcome::Win => Shape::Scissors,
                FightOutcome::Draw => Shape::Paper,
                FightOutcome::Loss => Shape::Rock,
            }
        }

        fights()
            .map(|(opponent, outcome)| {
                let you = misinterpret_as_shape(outcome);
                score(you, fight_outcome(you, opponent))
            })
            .sum()
    }

    pub fn part_2() -> u32 {
        fn pick_shape_for_outcome(opponent: Shape, outcome: FightOutcome) -> Shape {
            Shape::all()
                .find(|shape| fight_outcome(*shape, opponent) == outcome)
                .unwrap()
        }

        fights()
            .map(|(opponent, outcome)| {
                let you = pick_shape_for_outcome(opponent, outcome);
                score(you, outcome)
            })
            .sum()
    }
}

pub mod day_3 {
    use std::{collections::BTreeSet, io};

    fn priority(ch: char) -> u32 {
        match ch {
            'a'..='z' => 1 + ch as u32 - 'a' as u32,
            'A'..='Z' => 27 + ch as u32 - 'A' as u32,
            _ => panic!("och"),
        }
    }

    fn rucksacks() -> impl Iterator<Item = String> {
        io::stdin().lines().filter_map(Result::ok)
    }

    pub fn part_1() -> u32 {
        rucksacks()
            .map(|s| {
                let middle = s.len() / 2;
                let a = s[..middle].chars().collect::<BTreeSet<_>>();
                let b = s[middle..].chars().collect::<BTreeSet<_>>();
                *a.intersection(&b).next().unwrap()
            })
            .map(priority)
            .sum()
    }

    pub fn part_2() -> u32 {
        rucksacks()
            .array_chunks::<3>()
            .map(|arr| {
                let [a, b, c] = arr.map(|s| s.chars().collect::<BTreeSet<_>>());
                *a.intersection(&b).find(|ch| c.contains(*ch)).unwrap()
            })
            .map(priority)
            .sum()
    }
}

fn main() {
    let answer = day_3::part_2();
    println!("{answer}");
}
