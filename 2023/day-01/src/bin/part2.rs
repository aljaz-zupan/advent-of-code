use std::include_str;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("anwser: {}", output)
}

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
            println!("Number: {}", &number);
            if let Some(first_index) = line.find(&number) {
                let first_element = i32::try_from(first_index).expect("Number is to low or to high");
                println!("first_index {}", first_index);
                match left.value {
                    0 => {
                        left = return_as_number(&number, &first_element);
                        right = return_as_number(&number, &first_element )
                    }
                    _ => left = return_as_number(&number, &first_element),
                }
            } else if let Some(last_index) = line.rfind(&number) {
                let last_element = i32::try_from(last_index).expect("Number is to low");
                println!("last_index {}", last_index);
                if right.value == 0 {
                    right = return_as_number(&number, &last_element);
                }
            }
        }
        println!("{} {} {}", &line, &left.value, &right.value);
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
        let test_string = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let result = super::part2(test_string);
        assert_eq!(result, 281);
    }
}
