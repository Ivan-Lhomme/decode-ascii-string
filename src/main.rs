use std::io;

fn main() {
    let mut encode_str = String::new();
    let mut bytes = Vec::new();
    
    println!("Put the encode string :");
    io::stdin().read_line(&mut encode_str).expect("Failed to read line.");

    let encode_str = encode_str.trim();
    
    let mut i = 0;
    while i < encode_str.len() {
        let encode_char = &encode_str[i..i + 2];
        let byte = u8::from_str_radix(encode_char, 16).unwrap();

        bytes.push(byte);

        i += 2;
    }
    
    let decode_str = String::from_utf8(bytes).unwrap();

    println!("{}", decode_str)
}
