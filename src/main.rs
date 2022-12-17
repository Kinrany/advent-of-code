#![feature(iter_array_chunks)]

use std::{fmt, str::FromStr};

pub fn dbg<T: fmt::Debug>(s: &'static str) -> impl Fn(&T) {
    move |x| println!("{s}: {x:?}")
}

fn lines() -> impl Iterator<Item = String> {
    std::io::stdin().lines().flat_map(Result::ok)
}

pub mod day_1 {
    use std::mem;

    use super::*;

    fn inventories() -> impl Iterator<Item = u32> {
        let meals = lines().map(|line| line.trim().parse::<u32>().ok());
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
    use anyhow::Error;

    use super::*;

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
        lines().flat_map(|s| {
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
    use std::collections::BTreeSet;

    use super::*;

    fn priority(ch: char) -> u32 {
        match ch {
            'a'..='z' => 1 + ch as u32 - 'a' as u32,
            'A'..='Z' => 27 + ch as u32 - 'A' as u32,
            _ => panic!("och"),
        }
    }

    pub fn part_1() -> u32 {
        lines()
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
        lines()
            .array_chunks::<3>()
            .map(|arr| {
                let [a, b, c] = arr.map(|s| s.chars().collect::<BTreeSet<_>>());
                *a.intersection(&b).find(|ch| c.contains(*ch)).unwrap()
            })
            .map(priority)
            .sum()
    }
}

pub mod day_4 {
    use std::{
        num::ParseIntError,
        ops::{Deref, RangeInclusive},
    };

    use super::*;

    struct Assignment(RangeInclusive<u32>);

    impl FromStr for Assignment {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (a, b) = s.split_once('-').unwrap();
            let a: u32 = a.parse()?;
            let b: u32 = b.parse()?;
            Ok(Self(a..=b))
        }
    }

    impl Deref for Assignment {
        type Target = RangeInclusive<u32>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    struct AssignmentPair(Assignment, Assignment);

    impl FromStr for AssignmentPair {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (a, b) = s.split_once(',').unwrap();
            Ok(Self(a.parse()?, b.parse()?))
        }
    }

    pub fn part_1() -> u32 {
        impl Assignment {
            fn contains_other(&self, other: &Self) -> bool {
                self.start() <= other.start() && other.end() <= self.end()
            }
        }

        impl AssignmentPair {
            fn one_contains_other(&self) -> bool {
                self.0.contains_other(&self.1) || self.1.contains_other(&self.0)
            }
        }

        lines()
            .map(|s| AssignmentPair::from_str(&s))
            .filter_map(|a| a.ok())
            .filter(|a| a.one_contains_other())
            .count() as u32
    }

    pub fn part_2() -> u32 {
        impl AssignmentPair {
            fn overlap(&self) -> bool {
                self.0.start() <= self.1.end() && self.1.start() <= self.0.end()
            }
        }

        lines()
            .map(|s| AssignmentPair::from_str(&s))
            .filter_map(|a| a.ok())
            .filter(|a| a.overlap())
            .count() as u32
    }
}

pub mod day_5 {
    use std::{array, num::ParseIntError};

    use tracing::debug;

    use super::*;

    #[derive(Clone, Copy)]
    struct Move {
        count: usize,
        from: usize,
        to: usize,
    }

    impl FromStr for Move {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split_whitespace();
            let [_move, count, _from, from, _to, to] = array::from_fn(|_| s.next().unwrap());
            let (count, from, to) = (count.parse()?, from.parse()?, to.parse()?);
            Ok(Self { count, from, to })
        }
    }

    #[derive(Debug)]
    struct Stack(Vec<Vec<char>>);

    impl Stack {
        fn mov(&mut self, cmd: Move) -> String {
            (0..cmd.count)
                .map(|_| self.move_one(cmd.from, cmd.to))
                .collect()
        }

        fn move_one(&mut self, from: usize, to: usize) -> char {
            let ch = self.0[from].pop().unwrap();
            self.0[to].push(ch);
            ch
        }

        fn top(&self) -> String {
            self.0.iter().skip(1).map(|v| v.last().unwrap()).collect()
        }
    }

    impl fmt::Display for Stack {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (id, v) in self.0.iter().enumerate() {
                let line: String = v.iter().collect();
                writeln!(f, "{id}: {line}")?;
            }
            Ok(())
        }
    }

    pub fn part_1() -> impl fmt::Display {
        //                 [B]     [L]     [S]
        //         [Q] [J] [C]     [W]     [F]
        //     [F] [T] [B] [D]     [P]     [P]
        //     [S] [J] [Z] [T]     [B] [C] [H]
        //     [L] [H] [H] [Z] [G] [Z] [G] [R]
        // [R] [H] [D] [R] [F] [C] [V] [Q] [T]
        // [C] [J] [M] [G] [P] [H] [N] [J] [D]
        // [H] [B] [R] [S] [R] [T] [S] [R] [L]
        //  1   2   3   4   5   6   7   8   9
        let stack = Stack(vec![
            vec![],
            vec!['H', 'C', 'R'],
            vec!['B', 'J', 'H', 'L', 'S', 'F'],
            vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],
            vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],
            vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'],
            vec!['T', 'H', 'C', 'G'],
            vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'],
            vec!['R', 'J', 'Q', 'G', 'C'],
            vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S'],
        ]);
        debug!("{stack}");

        lines()
            .skip_while({
                // Skip all lines until (and including) the empty one
                let mut found_empty = false;
                move |line| {
                    if found_empty {
                        false
                    } else if line.is_empty() {
                        found_empty = true;
                        true
                    } else {
                        true
                    }
                }
            })
            .filter_map(|l| l.parse().ok())
            .fold(stack, |mut stack, mov| {
                let moved = stack.mov(mov);
                debug!(
                    "\nfrom {} to {} move {}: {moved}\n{stack}",
                    mov.from, mov.to, mov.count
                );
                stack
            })
            .top()
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let answer = day_5::part_1();
    println!("{answer}");
}
