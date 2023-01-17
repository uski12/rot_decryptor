use std::io::{self, Write};


fn main() {
    print!("Enter string to decrypt/encrypt: ");
    io::stdout()
        .flush()
        .expect("ERROR: Unable to flush.");
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("ERROR: Failed to read line.");
    str.make_ascii_uppercase();
    str.pop();

    for i in 1..26 {
        println!("Key {i}: {}", rotate(&str, i));
    };
}


fn rotate(string: &str, rot_factor: u32) -> String {
    let mut rot_str = String::new();
    for i in string.chars() {
        let ascii_code:u32 = i as u32;
        if i == ' ' || ascii_code < 65 || ascii_code > 90 {
            rot_str.push(i);
            continue;
        }
        let mut numeric_id = ascii_code + rot_factor - 64;
        numeric_id = (numeric_id)%26 + ((numeric_id%27)/26)*26;
        let rotated_char: char = char::from_u32(numeric_id + 64).unwrap();
        rot_str.push(rotated_char);
    };
    return rot_str;
}
