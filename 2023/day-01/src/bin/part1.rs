use std::include_str;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("anwser: {}", output)
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut line_sum = 0;
        let mut left = 0;
        let mut right = 0;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(num) => {
                    match (left, right) {
                        (0, 0) => {
                            left = num as i32;
                            right = num as i32;
                        }
                        (_, 0) => right = num as i32,
                        (0, _) => left = num as i32,
                        (_, _) => right = num as i32,
                    }
                    let left_str = &left.to_string();
                    let right_str = &right.to_string();
                    let line_string = format!("{}{}", left_str, right_str);
                    line_sum = line_string.parse().unwrap();
                }
                None => continue,
            }
        }
        sum += line_sum;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let test_string = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let result = super::part1(test_string);
        assert_eq!(result, 142);
    }
}
