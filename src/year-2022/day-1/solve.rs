use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

fn read_input_file_lines(file_path: String) -> Lines<BufReader<File>> {
    let result_open = File::open(file_path);
    let open_file = match result_open {
        Err(e) => panic!("Error opening input file: {e}"),
        Ok(file) => file,
    };

    let buffered_reader = BufReader::new(open_file);
    let lines = buffered_reader.lines();
    lines
}

fn update_top_three_if_warranted(max_totals: &mut Vec<u32>, current_elf_total_calories: &mut u32) {
    for index in 0..3 {
        let calories = max_totals[index];
        if *current_elf_total_calories > calories {
            let temp = calories;
            max_totals[index] = *current_elf_total_calories;
            *current_elf_total_calories = temp;
        }
    }
}

fn get_maximum_calories_for_top_three_elves(lines: Lines<BufReader<File>>) -> Vec<u32> {
    let mut current_elf_total_calories: u32 = 0;
    let mut max_totals: Vec<u32> = vec![0, 0, 0];

    for line in lines {
        let snack_calories: u32 = match line.unwrap().parse::<u32>() {
            Err(_e) => 0,
            Ok(calories) => calories,
        };

        if snack_calories > 0 {
            current_elf_total_calories += snack_calories;
            continue;
        }

        update_top_three_if_warranted(&mut max_totals, &mut current_elf_total_calories);
        current_elf_total_calories = 0;
    }

    update_top_three_if_warranted(&mut max_totals, &mut current_elf_total_calories);

    max_totals
}

fn get_part_1_solution(max_totals: &Vec<u32>) -> &u32 {
    &max_totals[0]
}

fn get_part_2_solution(max_totals: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for elf_total in max_totals {
        sum += elf_total
    }
    sum
}

pub fn solve(input_file_path: String) -> (String, String) {
    let lines = read_input_file_lines(input_file_path);
    let result = get_maximum_calories_for_top_three_elves(lines);

    let result_1 = get_part_1_solution(&result);
    let result_2 = get_part_2_solution(&result);

    (result_1.to_string(), result_2.to_string())
}

#[cfg(test)]
mod year_2022_day_1_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&["dev-input.txt", "24000", "45000"])]
    #[case(&["prod-input.txt", "71506", "209603"])]
    fn returns_correct_answers_for_parts_1_and_2(#[case] args: &[&str; 3]) {
        let args: Vec<String> = args.map(String::from).to_vec();
        let file_path = format!("src/year-2022/day-1/{}", args[0]);

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
