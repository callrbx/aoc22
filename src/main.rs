use std::time::Instant;
mod days;
mod util;

fn time_solve(func: fn() -> String) {
    let start = Instant::now();
    let day_name: String = func();
    println!("{}: {:.2?} ", day_name, start.elapsed())
}

fn main() {
    println!("Running All Day Solves:");

    time_solve(days::day1::solve);
    time_solve(days::day2::solve);
}
