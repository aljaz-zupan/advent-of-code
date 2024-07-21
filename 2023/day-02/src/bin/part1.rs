#[derive(Debug)]
struct Game {
    number: i32,
    max_draw: Draw,
}

#[derive(Debug)]
struct Draw {
    red: Option<i32>,
    green: Option<i32>,
    blue: Option<i32>,
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

fn main() {
    let input = include_str!("input.txt");
    let output = sum_game_numbers(input);
    println!("anwser: {}", output)
}

fn sum_game_numbers(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        //sum += get_g ¸came_number(line);
        let mut game = Game::new(line);
        println!("{}", game.print_game_number());
    }
    sum
}

impl Game {
    fn new(line: &str) -> Game {
        Game {
            number: Self::get_game_number(&line),
            //subsets: Self::return_subsets(&line),
            max_draw: Self::return_subsets(&line),
        }
    }

    //fn return_subsets(line: &str) -> Vec<Draw> {}

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

    fn max_draw(line: &str) {
        let lines = line.to_string().split("; ");

        for draw_line in lines {
            let colors = draw_line.split("; ");
        }
        return vec![];
    }

    fn print_game_number(&self) -> String {
        format!("The Game number is {}", self.number)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let line = "Game 11: 3 RED, 2 BLUE;\nGame 12: 4 RED, 3 GREEN";
        let result = 11 + 12;
        assert_eq!(result, 23);
    }
}
