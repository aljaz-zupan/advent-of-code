use std::include_str;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("anwser: {}", output)
}

struct Number {
    value: &str,
    index: i16,
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
            println!("Number: {}", &number);
            if let Some(first_index) = line.find(&number) {
                println!("first_index {}", first_index);
                match left.value {
                    0 => {
                        left = return_number(&number);
                        right = return_number(&number)
                    }
                    _ => left = return_number(&number),
                }
            } else if let Some(last_index) = line.rfind(&number) {
                println!("last_index {}", last_index);
                match right.value {
                    0 => right = return_number(&number),
                    _ => {}
                }
            }
        }
        println!("{} {} {}", &line, &left, &right);
        let line_string = format!("{}{}", &left.to_string(), &right.to_string());
        let line_sum: i32 = line_string.parse().unwrap();
        sum += line_sum
    }
    sum
}

fn return_number(number: &str) -> i32 {
    let the_number = match number {
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
    };
    the_number
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let test_string = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let result = super::part2(test_string);
        assert_eq!(result, 281);
    }
}
