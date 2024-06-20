fn main() {
    let input = include_str!("input.txt");
    let output = number_of_games(input);
    println!("anwser: {}", output)
}

fn number_of_games() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
