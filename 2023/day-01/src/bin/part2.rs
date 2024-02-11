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
    let numbers = vec!["1","2","3","4","5","6","7","8","9","one","two","three","four","five","six","seven","eight","nine"]

    for line in input.lines() {
        let mut line_sum = 0;
        let mut left = 0;
        let mut right = 0;
        
        
        for &number in &numbers {

            if let Some(first_index) = line.find(&number) {
                match left {
                    0 => {left = &number, right = &number},
                    _ => left = &number
                }

            }
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
