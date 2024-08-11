fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", &input.lines().count());
    let mut file = input.lines().into_iter().enumerate();
    println!("{:?}", file[3]);
}

#[allow(dead_code)]
fn find_symbol(line: &str) {
    let symbols: &[_] = &['+', '*', '%', '/', '@', '#', '-', '$', '&', '='];
    //let Some(symbol) = line.find(symbols);
}
