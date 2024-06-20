use std::include_str;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("anwser: {}", output)
}

#[derive(Debug)]
struct Number {
    value: i32,
    index: i32,
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let numbers = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    for line in input.lines() {
        let mut left: Number = Number {
            value: 0,
            index: 999,
        };
        let mut right: Number = Number {
            value: 0,
            index: -1,
        };

        for &number in &numbers {
            if let Some(first_index) = line.find(number) {
                let first_position = i32::try_from(first_index).expect("Index is too large to fit in an i32.");
                match left.value {
                    0 => {
                        left = return_as_number(number, &first_position);
                        right = return_as_number(number, &first_position);
                    }
                    _ => {
                        if left.index > first_position {
                            left = return_as_number(number, &first_position);
                        }
                        
                    }
                }
            }
            if let Some(last_index) = line.rfind(number) {
                let last_position = i32::try_from(last_index).expect("Index is too large to fit in an i32.");
                match right.value {
                    0 => right = return_as_number(number, &last_position),
                    _ => {
                        if right.index < last_position {
                            right = return_as_number(number, &last_position);
                        }
                    }
                }
            }
        }
        let line_string = format!("{}{}", &left.value.to_string(), &right.value.to_string());
        let line_sum: i32 = line_string.parse().unwrap();
        sum += line_sum
    }
    sum
}

fn return_as_number(number: &str, index: &i32) -> Number {
    Number {
        value: match number {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => 0,
        },
        index: *index,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let test_string = "two1nine\neightwothree\nabcone23xyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixten";

        let result = super::part2(test_string);
        assert_eq!(result, 281);
    }
}
