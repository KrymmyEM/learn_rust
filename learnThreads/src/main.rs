use std::thread;

fn main() {
    let th_one = thread::spawn(||{
        let mut val = String::new();
        print!("Please insert value:");
        std::io::stdin().read_line(&mut val).expect("Insert correct text");
        let _number: u32 = val.parse().expect("Not a number");
        let mut number = &_number;
        println!("{}",_number);
        println!("{}",number);
        loop {
            println!("{}",number);
        }
    });
    println!("Hello, world!");
}
