use std::io::stdin;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut s = String::new();

    stdin().read_to_string(&mut s).unwrap();

    let cnt = "abcdefghijklmnopqrstuvwxyz".chars().map(|c| (c, 0)).collect::<HashMap<_, _>>();

    for c in s.chars() {

    }
}
