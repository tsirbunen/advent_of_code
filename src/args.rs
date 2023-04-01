use std::env;

const ARGS_LENGTH_ERROR: &str = "\nYou must provide a year, a day and a mode. Example command:\n";
const CMD_EXAMPLE: &str = "\tcargo run -- 2020 1 dev\n";
const WRONG_YEAR_ERROR: &str = "\nYear must be 2020 or 2022!\n";
const PARSE_YEAR_ERROR: &str = "\nError parsing year!\n";
const WRONG_DAY_ERROR: &str = "\nDay must be between 1 and 25!\n";
const PARSE_DAY_ERROR: &str = "\nError parsing day!\n";
const PARSE_MODE_INFO: &str = "\nUnknown mode. Defaulting to dev.\n";

fn validate_args_length(args: &Vec<String>) {
    if args.len() < 4 {
        panic!("{ARGS_LENGTH_ERROR}{CMD_EXAMPLE}");
    }
}

fn get_year(args: &Vec<String>) -> u16 {
    match args[1].parse() {
        Ok(year) => match year {
            2020 => year,
            2022 => year,
            _ => panic!("{WRONG_YEAR_ERROR}"),
        },
        _ => panic!("{PARSE_YEAR_ERROR}"),
    }
}

fn get_day(args: &Vec<String>) -> u8 {
    match args[2].parse() {
        Ok(day) => match day {
            day if day > 0 && day < 26 => day,
            _ => panic!("{WRONG_DAY_ERROR}"),
        },
        _ => panic!("{PARSE_DAY_ERROR}"),
    }
}

fn get_input_file_name(args: &Vec<String>) -> String {
    match &args[3] {
        m if *m == String::from("prod") => String::from("prod-input.txt"),
        m if *m == String::from("dev") => String::from("dev-input.txt"),
        _ => {
            println!("{PARSE_MODE_INFO}");
            String::from("dev-input.txt")
        }
    }
}

///
/// Extracts the year, the day and the mode from arguments supplied by the user.
///
pub fn extract() -> (u16, String, String) {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", &args);
    validate_args_length(&args);

    let year: u16 = get_year(&args);
    let day: u8 = get_day(&args);
    let input_file_name: String = get_input_file_name(&args);
    let input_file_path: String = format!("src/year-{}/day-{}/{}", year, day, input_file_name);

    (year, day.to_string(), input_file_path)
}

#[cfg(test)]
mod args_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    #[should_panic(expected = "You must provide a year, a day and a mode")]
    fn too_few_arguments_cause_panic() {
        let args_mock = vec![String::from("target/debug/advent_of_code")];
        validate_args_length(&args_mock);
    }

    #[rstest]
    #[case(&["path", "2020", "5", "dev"])]
    #[case(&["path", "2020", "1", "prod"])]
    #[case(&["path", "2022", "25", "dev"])]
    fn extracts_correct_year(#[case] args: &[&str; 4]) {
        let args: Vec<String> = args.map(String::from).to_vec();
        let year = get_year(&args);
        let year_number: u16 = args[1].parse().unwrap();
        assert!(
            year == year_number,
            "Extracted year to be {} but got {}",
            year_number,
            year
        );
    }

    #[rstest]
    #[case(&["path", "2019", "1", "dev"])]
    #[case(&["path", "2023", "25", "prod"])]
    #[should_panic(expected = "2020 or 2022")]
    fn out_of_range_year_causes_panic(#[case] args: &[&str; 4]) {
        let args: Vec<String> = args.map(String::from).to_vec();
        get_year(&args);
    }

    #[rstest]
    #[case(&["path", "2020", "5", "dev"])]
    #[case(&["path", "2020", "1", "prod"])]
    #[case(&["path", "2022", "25", "dev"])]
    fn extracts_correct_day(#[case] args: &[&str; 4]) {
        let args: Vec<String> = args.map(String::from).to_vec();
        let day = get_day(&args);
        let day_number: u8 = args[2].parse().unwrap();
        assert!(
            day == day_number,
            "Extracted day to be {} but got {}",
            day_number,
            day
        );
    }

    #[rstest]
    #[case(&["path", "2022", "0", "dev"])]
    #[case(&["path", "2020", "26", "prod"])]
    #[should_panic(expected = "between 1 and 25")]
    fn out_of_range_day_causes_panic(#[case] args: &[&str; 4]) {
        let args: Vec<String> = args.map(String::from).to_vec();
        get_day(&args);
    }

    #[rstest]
    #[case(&"dev", &"dev-input.txt")]
    #[case(&"prod", &"prod-input.txt")]
    #[case(&"nonexisting", &"dev-input.txt")]
    fn extracts_correct_input_file_name(#[case] input_mode: &str, #[case] expected_name: &str) {
        let mut args: Vec<String> = ["path", "2020", "5"].map(String::from).to_vec();
        args.push(input_mode.to_string());
        let file_name = get_input_file_name(&args);
        let expected_name = expected_name.to_string();
        assert!(
            file_name == expected_name,
            "Extracted file name to be {} but got {}",
            expected_name,
            file_name
        );
    }
}
