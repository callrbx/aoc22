use std::time::{Duration, Instant};
mod days;
mod util;

fn time_solve(func: fn() -> (String, String)) -> Duration {
    let start = Instant::now();
    let (day, output) = func();
    let end = start.elapsed();
    println!("{}: {:.2?} {}", day, end, output);
    return end;
}

fn main() {
    let mut total: Duration = Duration::new(0, 0);

    println!("Running All Day Solves:");

    total += time_solve(days::day1::solve);
    total += time_solve(days::day2::solve);
    total += time_solve(days::day3::solve);
    total += time_solve(days::day4::solve);
    total += time_solve(days::day5::solve);
    total += time_solve(days::day6::solve);
    total += time_solve(days::day7::solve);

    println!("Total Time: {:.2?}", total);
}
