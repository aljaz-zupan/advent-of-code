#[derive(Clone, Debug, PartialEq, Eq)]
struct LineNumber {
    id: String,
    number: i32,
    index_range: Vec<usize>,
}

impl LineNumber {
    fn new(line_index: i32, numb: i32, range: &Vec<usize>) -> LineNumber {
        let range_string = range.into_iter().map(|n| n.to_string()).collect::<String>();
        return LineNumber {
            id: Self::generate_id(line_index, &numb, range_string),
            number: numb,
            index_range: range.to_vec(),
        };
    }

    fn generate_id(index: i32, index_range: &i32, range: String) -> String {
        let id: String = format!("{}-{}-{}", index, index_range, range).to_string();
        id
    }
}

fn main() {
    let input: &str = include_str!("input.txt");
    let symbols: Vec<char> = vec!['*'];

    let mut sum = 0;
    //let mut line_numbers: Vec<LineNumber> = vec![];

    let lines: Vec<&str> = input.lines().collect();
    let max = lines.len() - 1;

    for (line_index, line) in lines.iter().enumerate() {
        let line_index_i32: i32 = line_index.clone() as i32;

        // If first line then just continue since there are no symbols on firs line
        if line_index == 0 || line_index == max {
            continue;
        } else {
            for (char_index, char) in line.chars().enumerate() {
                let mut gear_sum: i32 = 0;
                if symbols.contains(&char) {
                    let top_line = lines.get((line_index_i32 - 1) as usize).unwrap();
                    let bottom_line = lines.get((line_index_i32 + 1) as usize).unwrap();

                    if let Some(numbers) =
                        return_numbers(char_index, line_index_i32, line, top_line, bottom_line)
                    {
                        if numbers.iter().len() == 2 {
                            print!("Numbers: {:?}", numbers);
                            let char_sum: i32 = numbers
                                .iter()
                                .map(|nu| nu.number)
                                .reduce(|acc, num| acc * num)
                                .unwrap_or(0);
                            gear_sum = char_sum;
                            print!(" Gear sum: {}", &char_sum)
                        }
                    }
                } else {
                    continue;
                }
                print!(" Before sum: {}", sum);
                sum += gear_sum;
                println!(" Sum: {}", sum);
            }
        }
    }

    println!(" sum: {}", sum);
}

fn return_numbers(
    index: usize,
    line_index: i32,
    line: &str,
    top: &str,
    bottom: &str,
) -> Option<Vec<LineNumber>> {
    let top_numbers = find_numbers(top, index, &(&line_index - 1)).unwrap();
    let bottom_numbers = find_numbers(bottom, index, &(&line_index + 1)).unwrap();
    let middle_numbers = find_numbers(line, index, &line_index).unwrap();

    let all_numbers: Vec<LineNumber> = top_numbers
        .into_iter()
        .chain(middle_numbers.into_iter())
        .chain(bottom_numbers.into_iter())
        .collect();

    return Some(all_numbers);
}

fn find_numbers(string: &str, index: usize, line_index: &i32) -> Option<Vec<LineNumber>> {
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
                process_digits(&mut temp, line_index, &mut temp_index, &mut num, &index);
            }
            temp_index.clear();
            temp.clear();
        }
    }

    // Process any remaining digits at the end of the line
    if !temp.is_empty() {
        process_digits(&mut temp, line_index, &mut temp_index, &mut num, &index);
    }

    Some(num)
}

fn process_digits(
    temp: &mut Vec<char>,
    line_index: &i32,
    temp_index: &mut Vec<usize>,
    num: &mut Vec<LineNumber>,
    index: &usize,
) {
    let collection: String = temp.iter().collect();
    let temp_line_num = LineNumber::new(
        line_index.clone() as i32,
        collection.parse::<i32>().unwrap(),
        &temp_index,
    );

    let left_index = index - 1;
    let right_index = index + 1;

    if temp_line_num
        .index_range
        .iter()
        .any(|&i| i >= left_index && i <= right_index)
    {
        num.push(temp_line_num);
    }
}
