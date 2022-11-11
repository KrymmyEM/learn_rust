use std::{thread, io::{Read, Write, self, stdin, stdout}};
fn main() {
    let mut u8CodeChar= [0u8];
    stdin().read(&mut u8CodeChar).unwrap();
    stdout().flush().unwrap();
    
    print!("U8:{} Char:{}\n", u8CodeChar[0], std::str::from_utf8(&u8CodeChar).unwrap());
    
    let mut changedChar = std::str::from_utf8(&u8CodeChar).unwrap().to_lowercase();
    if std::str::from_utf8(&u8CodeChar).unwrap() == std::str::from_utf8(&u8CodeChar).unwrap().to_lowercase(){
        changedChar = std::str::from_utf8(&u8CodeChar).unwrap().to_uppercase();
    }
    let mut u8CodeChar = changedChar.as_bytes();
    print!("U8:{} Char:{}\n", u8CodeChar[0], changedChar);
}
