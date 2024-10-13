use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
struct LineNumber {
    id: i32,
    string: i32,
    index_range: Vec<usize>,
}

impl LineNumber {
    fn new(line_index: i32, numb: i32, range: &Vec<usize>) -> LineNumber {
        return LineNumber {
            id: line_index,
            string: numb,
            index_range: range.to_vec(),
        };
    }

    fn generate_id(index, index_range) {

    }
}

fn main() {
    let input: &str = include_str!("input.txt");
    let symbols: Vec<char> = vec!['+', '*', '%', '/', '@', '#', '-', '$', '&', '='];

    let mut sum = 0;
    let mut lineNumbers: HashMap<LineNumber>;

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

                    if let Some(numbers) =
                        return_numbers(char_index, &line_index, line, top_line, bottom_line)
                    {
                        println!("Numbers: {:?}", numbers);

                        for number in numbers {
                            sum = sum + number.string;
                            lineNumbers.append(other);
                        }
                    }
                }
            }
        }
        println!("");
    }

    println!("sum: {}", sum);
}

fn return_numbers(
    index: usize,
    line_index: &usize,
    line: &str,
    top: &str,
    bottom: &str,
) -> Option<Vec<LineNumber>> {
    let top_numbers = find_numbers(top, index, &line_index - 1 ).unwrap();
    let bottom_numbers = find_numbers(bottom, index, &line_index + 1).unwrap();
    let middle_numbers = find_numbers(line, index, &line_index).unwrap();

    let all_numbers: Vec<LineNumber> = top_numbers
        .into_iter()
        .chain(middle_numbers.into_iter())
        .chain(bottom_numbers.into_iter())
        .collect();

    //dbg!(top_numbers, bottom_numbers, middle_numbers);
    return Some(all_numbers);
}

fn find_numbers(string: &str, index: usize, line_index: &usize) -> Option<Vec<LineNumber>> {
    let numbers = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let string: String = string.to_string();

    let mut num: Vec<LineNumber> = vec![];
    let mut temp: Vec<char> = vec![];
    let mut temp_index: Vec<usize> = vec![];

    for (ci, char) in string.chars().enumerate() {
        if numbers.contains(&char) {
            temp.push(char.clone());
            temp_index.push(ci);
        } else {
            if !temp.is_empty() {
                let collection: String = temp.iter().collect();
                let temp_line_num =
                    LineNumber::new(line_index, collection.parse::<i32>().unwrap(), &temp_index);
                let left_index = index - 1;
                let right_index = index + 1;

                /* If any of the index number is in the range if the symbol index then push to numbers */
                if temp_line_num
                    .index_range
                    .iter()
                    .any(|&i| i >= left_index && i <= right_index)
                {
                    num.push(temp_line_num);
                }
            }
            temp_index.clear();
            temp.clear();
        }
    }

    Some(num)
}
