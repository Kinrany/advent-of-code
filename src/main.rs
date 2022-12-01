use std::{io, mem};

fn main() {
    // day 1 part 1
    let lines = io::stdin().lines().map(Result::unwrap);
    let meals = lines.map(|line| line.trim().parse::<u32>().ok());
    let inventories = meals
        .chain([None])
        .scan(0, |inv, meal| match meal {
            Some(meal) => {
                *inv += meal;
                Some(None)
            }
            None => Some(Some(mem::take(inv))),
        })
        .flatten();
    let answer = inventories.max().unwrap();
    println!("{answer}");
}
