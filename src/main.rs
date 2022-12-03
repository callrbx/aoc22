use std::time::Instant;
mod days;
mod util;

fn time_solve(func: fn() -> (String, String)) {
    let start = Instant::now();
    let (day, output) = func();
    println!("{}: {:.2?} {}", day, start.elapsed(), output)
}

fn main() {
    println!("Running All Day Solves:");

    time_solve(days::day1::solve);
    time_solve(days::day2::solve);
    time_solve(days::day3::solve);
}
