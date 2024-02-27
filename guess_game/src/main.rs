use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    // take input
    io::stdin()
        .read_line(&mut input)
        .expect("hellos");

    let num: i32  =  input.trim().parse().expect("not a number");
    let hard_coded_num = 45;
    match  num.cmp(&hard_coded_num){
        Ordering::Less => print_result("Number is less"),
        Ordering::Equal => print_result("Number is Equal"),
        Ordering::Greater => print_result("Number is More"),
    }


}

fn print_result(str: &str){
    println!("{str}")
}
