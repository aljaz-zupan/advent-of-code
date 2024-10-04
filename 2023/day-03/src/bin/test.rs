#![allow(unused)]
fn main() {
    let a = [1, 2, 3];

    assert!(a.iter().any(|&x| x > 0));

    assert!(a.iter().any(|&x| x > 5));
}
