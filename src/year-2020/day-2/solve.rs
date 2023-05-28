use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Lines;

fn read_input_file_lines(file_path: String) -> Lines<BufReader<File>> {
    let result = File::open(file_path);
    let file = match result {
        Err(e) => panic!("Error opening input file: {e}"),
        Ok(file) => file,
    };

    let buffered_reader = BufReader::new(file);
    let lines = buffered_reader.lines();
    lines
}

fn parse_line(line: Result<String, Error>) -> (u32, u32, char, String) {
    let line_data = line.unwrap();
    let items = line_data
        .split(|separator| separator == ' ' || separator == '-' || separator == ':')
        .collect::<Vec<&str>>();

    let first_number: u32 = items[0].parse().expect("Failed to parse first number");
    let second_number: u32 = items[1].parse().expect("Failed to parse second number");
    let character: char = items[2].parse().expect("Failed to parse character");
    let target: String = items[4].parse().expect("Failed to parse target");
    (first_number, second_number, character, target)
}

fn parse_lines(lines: Lines<BufReader<File>>) -> Vec<(u32, u32, char, String)> {
    let mut parsed_lines: Vec<(u32, u32, char, String)> = Vec::new();
    for line in lines {
        parsed_lines.push(parse_line(line));
    }
    parsed_lines
}

fn count_passwords_with_correct_number_of_characters(lines: &Vec<(u32, u32, char, String)>) -> u32 {
    let mut total_count: u32 = 0;

    for line in lines {
        let (minimum, maximum, character, target) = line;

        let mut characters_count: u32 = 0;
        for c in target.chars() {
            if c == *character {
                characters_count += 1;
            }
        }

        if minimum <= &characters_count && characters_count <= *maximum {
            total_count += 1;
        }
    }

    total_count
}

fn count_passwords_with_characters_in_correct_locations(
    lines: &Vec<(u32, u32, char, String)>,
) -> u32 {
    let mut total_count: u32 = 0;

    for line in lines {
        let (first_index, second_index, character, target) = line;
        let characters = target.chars().collect::<Vec<char>>();
        let first = first_index - 1;
        let is_at_first_index = characters[first as usize] == *character;
        let second = second_index - 1;
        let is_at_second_index = characters[second as usize] == *character;
        if is_at_first_index != is_at_second_index {
            total_count += 1;
        }
    }

    total_count
}

pub fn solve(input_file_path: String) -> (String, String) {
    let lines = read_input_file_lines(input_file_path);
    let parsed_lines = parse_lines(lines);
    let count_part_1 = count_passwords_with_correct_number_of_characters(&parsed_lines);
    let count_part_2 = count_passwords_with_characters_in_correct_locations(&parsed_lines);

    (count_part_1.to_string(), count_part_2.to_string())
}

#[cfg(test)]
mod year_2020_day_2_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&["dev-input.txt", "2", "1"])]
    #[case(&["prod-input.txt", "536", "558"])]
    fn returns_correct_answers_for_parts_1_and_2(#[case] args: &[&str; 3]) {
        let args: Vec<String> = args.map(String::from).to_vec();
        let file_path = format!("src/year-2020/day-2/{}", args[0]);

        let part_1_expected: String = String::from(&args[1]);
        let part_2_expected: String = String::from(&args[2]);
        let (part_1_result, part_2_result) = solve(file_path);
        assert!(
            part_1_result == part_1_expected,
            "Expected part 1 result to be {} but got {}",
            part_1_expected,
            part_1_result
        );
        assert!(
            part_2_result == part_2_expected,
            "Expected part 2 result to be {} but got {}",
            part_2_expected,
            part_2_result
        );
    }
}
