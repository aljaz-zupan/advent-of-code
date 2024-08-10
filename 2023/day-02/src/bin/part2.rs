use std::char::ToLowercase;

#[derive(Debug)]
struct Game {
    number: i32,
    red: i32,
    green: i32,
    blue: i32,
}

struct MaxGame {
    red: i32,
    green: i32,
    blue: i32,
}

enum Color {
    RED,
    GREEN,
    BLUE,
}

const BAG_CONTAINS: MaxGame = MaxGame {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let input = include_str!("input.txt");
    let output = sum_game_numbers(input);
    println!("anwser: {}", output)
}

fn sum_game_numbers(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        //sum += get_g ¸came_number(line);
        let mut game = Game::new(&line);
        //println!("{}", game.print_game_number());
        game.max_draw(&line);
        sum += game.power_of_draw()
        //println!("Game: {:?}", game)
    }
    sum
}

impl Game {
    fn new(line: &str) -> Game {
        Game {
            number: Self::get_game_number(&line),
            red: 0,
            green: 0,
            blue: 0,
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

    fn max_draw(&mut self, game: &str) {
        //println!("game {:?}", &game);
        if let Some(end) = &game[0..].find(':') {
            let draws = game[end + 1..].split("; ");

            //println!("draws {:?}", &draws);

            for draw in draws {
                //println!("color str: {}", &draw);
                let draw_color = draw.split(',');
                for color in draw_color {
                    let (num, color_name) = Self::return_color_and_number(color.trim());
                    //println!("num: {}; color: {};", &num, &color_name);
                    match color_name {
                        "red" => {
                            if self.red < num {
                                self.red = num;
                            }
                        }
                        "green" => {
                            if self.green < num {
                                self.green = num;
                            }
                        }
                        "blue" => {
                            if self.blue < num {
                                self.blue = num;
                            }
                        }
                        _ => panic!("Not a color"),
                    }
                }
            }
        }
    }

    fn power_of_draw(&self) -> i32 {
        self.red * self.green * self.blue
    }

    fn is_within_max(&self, max: MaxGame) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    fn return_color_and_number(color_string: &str) -> (i32, &str) {
        let mut parts = color_string.splitn(2, ' ');

        if let (Some(num_str), Some(color)) = (parts.next(), parts.next()) {
            match num_str.trim().parse::<i32>() {
                Ok(num) => (num, color),
                Err(_) => panic!("Failed to parse the number"),
            }
        } else {
            panic!("The input does not contain exactly two parts separated by ' '");
        }
    }

    fn print_game_number(&self) -> String {
        format!("The Game number is {}", self.number)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let line = "Game 11: 3 RED, 2 BLUE;\nGame 12: 4 RED, 3 GREEN".to_string();
        let result = 11 + 12;
        assert_eq!(result, 23);
    }
}
