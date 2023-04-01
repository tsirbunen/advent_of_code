#[path = "args.rs"]
mod args;
#[path = "year-2020.rs"]
mod year_2020;
#[path = "year-2022.rs"]
mod year_2022;

fn print_results(results: (String, String)) {
    println!("Results:");
    println!("  - part 1: {}", results.0);
    println!("  - part 2: {}\n", results.1);
}

///
/// The main handle for the project with which to run the challenges for different days.
///
/// # Usage example
/// Run in terminal with command
///
/// `cargo run -- <year> <day> <mode>`
///
/// - year is 2020 or 2022
/// - day is 1-25
/// - mode is `dev` or `prod`
///
fn main() {
    let (year, day, input_file_path) = args::extract();
    let results: (String, String) = match year {
        2020 => year_2020::solve_challenge(day, input_file_path),
        2022 => year_2022::solve_challenge(day, input_file_path),
        _ => panic!("\nSolutions to challenges not implemented for this year!\n"),
    };
    print_results(results);
}

#[cfg(test)]
mod main_tests {
    use rstest::rstest;
    use std::process::Command;

    #[rstest]
    #[case(&["run", "--", "2020", "1", "dev"])]
    #[case(&["run", "--", "2020", "1", "prod"])]
    fn program_can_be_run_with_proper_args(#[case] args: &[&str; 5]) {
        let output = Command::new("cargo").args(args).output().unwrap();
        assert!(output.status.success());
    }

    #[rstest]
    #[case(&vec!["run", "--", "2029", "1"])]
    #[case(&vec!["run", "--"])]
    #[case(&vec!["run"])]
    #[case(&vec!["run", "--", "2020", "1"])]
    #[case(&vec!["run", "--", "2020", "0"])]
    #[case(&vec!["run", "--", "2020", "26"])]
    #[case(&vec!["run", "--", "1", "2020", "dev"])]
    fn program_fails_with_improper_args(#[case] args: &Vec<&str>) {
        let output = Command::new("cargo").args(args).output().unwrap();
        assert!(
            output.status.code().unwrap_or(1) != 0,
            "Expected error but got successful outcome!"
        );
    }
}
