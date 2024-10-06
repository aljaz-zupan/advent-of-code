#![allow(unused)]
fn main() {
    let a = [232, 233, 234];

    if a.iter().any(|&x| x <= 234 && x >= 232) {
        println!("yes")
    } else {
        println!("no")
    }

    assert!(a.iter().any(|&x| x > 5));
}
