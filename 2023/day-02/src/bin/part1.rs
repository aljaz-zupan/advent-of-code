fn main() {
    let input = include_str!("input.txt");
    let output = sum_game_numbers(input);
    println!("anwser: {}", output)
}

fn sum_game_numbers(input: &str) -> i32 {

    let mut sum = 0;

    for line in input.lines() {
        sum += get_game_number(line);
    }
    sum
}

impl Game {

    fn new(line: &str) -> Game {
        Game { number: (), subsets: () }
    }

    fn get_game_number(line: &str) -> i32 {
        if let Some(start) = line.find("Game ") {
            if let Some(end) = line[start..].find(':') {
                let number_str = &line[5..end];
                if let Ok(number) = number_str.parse::<i32>() {
                    number
                } else {
                    0
                }
            } else {
                panic!("Does not have an end");
            }
        } else {
            panic!("Does not have a start");
        }
        
    }
}

fn return_all_plays(line: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let line = "Game 11: 3 RED, 2 BLUE;\nGame 12: 4 RED, 3 GREEN";
        let result = get_game_number(line);
        assert_eq!(result, 23);
    }
}

#[derive(Debug)]
struct Game {
    number: i32,
    subsets: Vec<Draw>
}

#[derive(Debug)]
struct Draw {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>
}

enum Color {
    RED,
    GREEN,
    BLUE
}

struct Play {
    color: Color,
    nummber: i32
}
