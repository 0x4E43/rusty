use std::io;

fn main() {
    println!("Hello, world!");

    let mut buf = String::new();
    // take input
    io::stdin()
        .read_line(&mut buf)
        .expect("hellos");


    println!("You entered : {buf}")

}
