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

    let file = input.lines().enumerate();
    let max: i32 = i32::try_from(file.clone().count()).unwrap() - 1;

    for (line_index, line) in file {
        //let index = i32::try_from(line_index).unwrap();

        // If first line then just continue since there are no symbols on firs line
        if line_index == usize::try_from(0).unwrap() || line_index == usize::try_from(max).unwrap()
        {
            continue;
        } else {
            print!("{} ->", &line_index);
            //println!("{}: {}", &line_index, &line);
            for (char_index, char) in line.chars().enumerate() {
                if symbols.contains(&char) {
                    print!("{}[{}], ", char, char_index);
                }
            }
        }
        println!("");
    }
}

/*fn top_line(line: &str, index: usize): Option<Vec<i32>> {

}*/
