use std::i32;

struct Symbol_neighbours {
    symbol: char,
    symbol_index: usize,
    current_line: String,
    next_line: Option<String>,
    previous_line: Option<String>,
}

fn main() {
    let input = include_str!("input.txt");
    let symbols: &[_] = &['+', '*', '%', '/', '@', '#', '-', '$', '&', '='];

    let mut file = input.lines().into_iter().enumerate();

    for (line_index, line) in file {
        let index = i32::try_from(line_index).unwrap();
        let max = i32::try_from(file.copy.count()).unwrap() - 1;
        // If first line then just continue since there are no symbols on firs line
        if (index == 0 || index == max) {
            continue;
        } else {
            println!("{}", &line);
        }
    }
}
