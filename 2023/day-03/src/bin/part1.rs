use std::{iter::Enumerate, str::Lines};

struct SymbolNeighbours {
    symbol: char,
    symbol_index: usize,
    current_line: String,
    next_line: Option<String>,
    previous_line: Option<String>,
}

fn main() {
    let input: &str = include_str!("input.txt");
    let symbols: Vec<char> = vec!['+', '*', '%', '/', '@', '#', '-', '$', '&', '='];

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
                    let top_line = lines.get(line_index - 1);
                    let bottom_line = lines.get(line_index + 1);

                    let m

                    print!(
                        "{}[{}], top: {:?}, bottom:{:?} , ",
                        char, char_index, top, bottom
                    );
                }
            }
        }
        println!("");
    }
}

fn get_numbers(index: usize, line: &str) : Option<Vec<i32>> {

}
