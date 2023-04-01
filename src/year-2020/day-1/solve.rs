use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn read_input_file_contents(file_path: String) -> String {
    let result = fs::read_to_string(file_path);
    let contents: String = match result {
        Err(e) => panic!("Error in reading input file: {e}"),
        Ok(data) => data,
    };
    contents
}

fn arrange_numbers_into_vector(file_contents: String) -> Vec<u32> {
    let numbers: Vec<u32> = file_contents
        .split("\n")
        .map(|number| number.parse::<u32>().expect("Error parsing input numbers"))
        .collect();

    numbers
}

fn find_pair(numbers: &Vec<u32>) -> Vec<u32> {
    let mut unpaired: HashSet<u32> = HashSet::new();
    for number in numbers {
        let pair = 2020 - number;
        if unpaired.contains(&pair) {
            return vec![number.clone(), pair];
        }
        unpaired.insert(number.clone());
    }

    panic!("Should have found a pair but didn't!");
}

fn find_triplet(numbers: &Vec<u32>) -> Vec<u32> {
    let mut singles: Vec<u32> = vec![];
    let mut pairs: HashMap<u32, (u32, u32)> = HashMap::new();
    for number in numbers {
        let target = 2020 - number;
        if pairs.contains_key(&target) {
            let (first, second) = pairs[&target];
            return vec![number.clone(), first, second];
        };
        for single in &singles {
            let sum = single + number;
            pairs.insert(sum, (single.clone(), number.clone()));
        }
        singles.push(number.clone());
    }
    panic!("Should have found a triplet but didn't!");
}

fn calculate_result(numbers: Vec<u32>) -> u32 {
    let mut multiplication = 1;
    for number in numbers {
        multiplication *= number;
    }
    multiplication
}

pub fn solve(input_file_path: String) -> (String, String) {
    let file_contents = read_input_file_contents(input_file_path);
    let numbers = arrange_numbers_into_vector(file_contents);

    // Solve part 1:
    let pair = find_pair(&numbers);
    let result_1 = calculate_result(pair);

    // Solve part 2:
    let triplet = find_triplet(&numbers);
    let result_2 = calculate_result(triplet);

    (result_1.to_string(), result_2.to_string())
}

#[cfg(test)]
mod year_2020_day_1_tests {
    use super::*;

    #[test]
    fn returns_correct_dev_answer() {
        let file_path: String = String::from("src/year-2020/day-1/dev-input.txt");
        let dev_expected_result: String = String::from("514579");
        let (dev_result, _prod_result) = solve(file_path);
        assert!(
            dev_result == dev_expected_result,
            "Expected dev result to be {} but got {}",
            dev_expected_result,
            dev_result
        );
    }
}
