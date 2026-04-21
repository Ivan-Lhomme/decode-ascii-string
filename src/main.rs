use std::io;

fn main() {
    let mut encode_str = String::new();

    println!("Put the encode string :");
    io::stdin().read_line(&mut encode_str).expect("Failed to read line.");

    println!("{}", encode_str)
}
