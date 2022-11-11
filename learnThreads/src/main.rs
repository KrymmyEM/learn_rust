use std::{thread, io::{stdin, stdout}};
fn main() {
    println!("Insert count threads");
    let mut value = String::new();
    stdin().read_line(&mut value).unwrap();
    let count_threads:i32 = value.trim().parse().unwrap();
    for iter in 1..count_threads+1{
        let mut neThread = thread::spawn(||{
            let mut i:u32 = 0;
            loop {
                println!("{}", i);
                i += 1;
                thread::sleep(std::time::Duration::from_secs(1));
            }
        });
        thread::sleep(std::time::Duration::from_secs(2));
    }
}
