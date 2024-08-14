fn main() {
    let input = include_str!("input.txt");
    let symbols: &[_] = &['+', '*', '%', '/', '@', '#', '-', '$', '&', '='];

    for (line_number, line) in input.lines().into_iter().enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if symbols.contains(&char) {
                print!("{} at {}, ", char, char_index);
            }
        }
        print!("\n");
    }
}
