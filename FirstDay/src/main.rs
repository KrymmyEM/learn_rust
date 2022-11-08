extern crate rand;
use rand::Rng;
use std::{thread, time};


struct Point{
    x: i32,
    y: i32,
}

fn main() {
    let width: i32 = 10;
    let height: i32 = 10;
    let mut point = Point{x:8, y:8};
    let mut gX: i32;
    let mut gY: i32;
    let millis = time::Duration::from_millis(100);
    let now = time::Instant::now();
    loop{
        for gX in 0..width+1{
            for gY in 0..height+1{
                if gY == point.y && gX == point.x{
                    print!(" * ");
                    continue;
                }
                if gX == width{
                    print!(" = ");
                }
                else if gY == height{
                    print!(" = \n");
                    continue;
                }
                if (gY != 0 && gY != height) && gX == 0{
                    print!(" = ");
                    continue;
                }
                if gY == 0{
                    print!(" = ");
                    continue;
                }
                if (gY > 0 && gY < height) && (gX >= 0 && gX < width){
                    print!(" . ");
                }
            }
        }
        point.x = rand::thread_rng().gen_range(1, width-1);
        point.y = rand::thread_rng().gen_range(1, height-1);
        print!("{}c",27 as char);
        thread::sleep(millis);
    }
}