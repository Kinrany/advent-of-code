use std::{fmt, io, mem};

pub fn dbg<T: fmt::Debug>(s: &'static str) -> impl Fn(&T) {
    move |x| println!("{s}: {x:?}")
}

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

pub fn day_1_part_1() {
    let answer = inventories().max().unwrap();
    println!("{answer}");
}

pub fn day_1_part_2() {
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

fn main() {
    day_1_part_2()
}
