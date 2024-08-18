fn main() {
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let string: String = "....573.613.........965............691......892..948.......964........439.375..................320......273...........352.284...............".to_string();

    let mut num: Vec<char> = vec![];

    for (index, char) in string.chars().enumerate() {
        if numbers.contains(&char) {
            println!("[{}]{}", index, char);
        }
    }
}
