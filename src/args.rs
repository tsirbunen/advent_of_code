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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn extracts_correct_day_from_args() {
//         let args_mock = vec![
//             String::from("target/debug/advent_of_code"),
//             String::from("2020"),
//             String::from("5"),
//             String::from("dev"),
//         ];
//         let day = get_day(&args_mock);
//         assert!(day == 5, "Extracted day should have been 5 but was {}", day);
//     }

//     #[test]
//     #[should_panic(expected = "You must provide a year, a day and a mode")]
//     fn too_few_arguments_cause_panic() {
//         let args_mock = vec![String::from("target/debug/advent_of_code")];
//         validate_args_length(&args_mock);
//     }
// }
