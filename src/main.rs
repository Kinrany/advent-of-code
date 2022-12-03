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

    pub fn part_1() {
        let answer = inventories().max().unwrap();
        println!("{answer}");
    }

    pub fn part_2() {
        let answer: u32 = inventories()
            .fold([0, 0, 0], |mut acc, inv| {
                let min = *acc.iter().min().unwrap();
                if min < inv {
                    *acc.iter_mut().find(|x| **x == min).unwrap() = inv;
                }
                acc
            })
            .iter()
            .sum();
        println!("{answer}");
    }
}

pub mod day_2 {
    use std::{io, str::FromStr};

    use anyhow::{bail, Error};

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
    }

    impl FromStr for Shape {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" | "X" => Ok(Self::Rock),
                "B" | "Y" => Ok(Self::Paper),
                "C" | "Z" => Ok(Self::Scissors),
                _ => bail!("oof"),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
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

    #[derive(Clone, Copy, Debug)]
    struct Fight {
        opponent: Shape,
        you: Shape,
    }

    impl Fight {
        fn outcome(self) -> FightOutcome {
            use Shape::*;

            // TODO: generalize if there are more
            match (self.you, self.opponent) {
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

        fn score(self) -> u32 {
            self.you.score() + self.outcome().score()
        }
    }

    impl FromStr for Fight {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (a, b) = s.split_once(' ').ok_or_else(|| Error::msg("foo"))?;
            Ok(Self {
                opponent: a.parse()?,
                you: b.parse()?,
            })
        }
    }

    pub fn part_1() {
        let answer = io::stdin()
            .lines()
            .flat_map(Result::ok)
            .map(|s| Fight::from_str(&s))
            .flat_map(Result::ok)
            .map(Fight::score)
            .sum::<u32>();
        println!("{answer}");
    }
}

fn main() {
    day_2::part_1()
}
