use std::io;

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Bro I can't even read that input ???");
    input.trim().to_string()
}