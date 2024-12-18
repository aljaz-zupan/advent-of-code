fn main() {
    let input: &str = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let sum = lines.iter().map(|line: &&str| {
        let data: Vec<&str> = line.split(':').collect();
        let card_number = data[0];
        let line_numbers: Vec<&str> = data[1].split('|').collect();
        let chosen_numbers: Vec<&str> = line_numbers[0].split(' ').collect();
        let card_numbers: Vec<&str> = line_numbers[1].split(' ').collect();
    });
    println!("sdsas");
}
