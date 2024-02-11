use std::include_str;

enum Numbers {

}

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    println!("anwser: {}", output)
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut line_sum = 0;
        let mut left = 0;
        let mut right = 0;
        
        match line {
            ()
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let test_string = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let result = super::part1(test_string);
        assert_eq!(result, 142);
    }
}
