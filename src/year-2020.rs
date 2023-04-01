#[path = "year-2020/day-1/solve.rs"]
mod solve_01;

///
/// Handles calling the correct day's module for solving the challenge (year 2020).
///
/// - There is a separate solver module for each day.
/// - Each module exposes the solve function to solve the day's challenge.
///
pub fn solve_challenge(day: String, input_file_path: String) -> (String, String) {
    let day: u8 = day.parse::<u8>().unwrap();

    match day {
        1 => solve_01::solve(input_file_path),
        _ => panic!("Solution to the requested day's challenge not implemented yet!"),
    }
}
