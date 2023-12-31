#![feature(iter_array_chunks, get_many_mut)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::wildcard_imports,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation
)]

pub mod year_2022;

fn main() {
    tracing_subscriber::fmt::init();
    let answer = year_2022::day_6::part_2();
    println!("{answer}");
}
