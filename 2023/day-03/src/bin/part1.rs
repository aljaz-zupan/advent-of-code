struct SymbolNeighbours {
    symbol: char,
    symbol_index: usize,
    current_line: String,
    next_line: Option<String>,
    previous_line: Option<String>,
}

enum Direction {
    Left,
    Right,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let symbols: Vec<char> = vec!['+', '*', '%', '/', '@', '#', '-', '$', '&', '='];

    let mut sum = 0;

    let lines: Vec<&str> = input.lines().collect();
    let max = lines.len() - 1;

    for (line_index, line) in lines.iter().enumerate() {
        //let index = i32::try_from(line_index).unwrap();

        // If first line then just continue since there are no symbols on firs line
        if line_index == 0 || line_index == max {
            continue;
        } else {
            print!("{} ->", &line_index);
            //println!("{}: {}", &line_index, &line);
            for (char_index, char) in line.chars().enumerate() {
                if symbols.contains(&char) {
                    let top_line = lines.get(line_index - 1).unwrap();
                    let bottom_line = lines.get(line_index + 1).unwrap();

                    if Some(numbers) = return_numbers(char_index, line, top_line, bottom_line) {

                        /*print!(
                        "{}[{}], top: {:?}, bottom:{:?} , ",
                        char, char_index, top, bottom
                        );*/
                    }
                }
            }
        }
        println!("");
    }
}

fn return_numbers(index: usize, line: &str, top: &str, bottom: &str) -> Option<Vec<i32>> {
    let numbers: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let middle_vec: Vec<char> = line.chars().collect();
    let top_vec: Vec<char> = top.chars().collect();
    let bottom_vec: Vec<char> = top.chars().collect();

    Some(vec![1, 2, 3])
}

fn find_number(line: &str, index: usize, direction: Direction) {}
