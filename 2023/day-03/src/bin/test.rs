struct Digit {
    current_index: usize,    // Current index of the last found char of a digit
    digit_string: Vec<char>, // Add char to string if is next to previus char
    direction: Direction,    // Current dirrection of itterator
}
enum Direction {
    Left,
    Right,
}

fn main() {
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let string: String = "....573.613.........965............691......892..948.......964........439.375..................320......273...........352.284...............".to_string();

    let mut num: Vec<String> = vec![];
    let mut temp: Vec<char> = vec![];

    for (index, char) in string.chars().enumerate() {
        if numbers.contains(&char) {
            temp.push(char.clone());
        } else {
            if !temp.is_empty() {
                let collection: String = temp.iter().collect();
                num.push(collection);
                println!("[{}]{}", index, char.clone());
            }
            temp.clear();
        }
    }

    println!("num: {:?}", num)
}
