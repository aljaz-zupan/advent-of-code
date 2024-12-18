#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
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
        let mut game = Game::new();
        //println!("{}", game.print_game_number());
        game.max_draw(&line);
        sum += game.power_of_draw()
        //println!("Game: {:?}", game)
    }
    sum
}

impl Game {
    fn new() -> Game {
        Game {
            red: 0,
            green: 0,
            blue: 0,
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
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 11 + 12;
        assert_eq!(result, 23);
    }
}
