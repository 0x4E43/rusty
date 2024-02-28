use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    //generate a random number between 0-100
    let hard_coded_num = rand::thread_rng().gen_range(1..=100);
    println!("Hello, world! {hard_coded_num}");

    let mut input = String::new();
    // take input
    io::stdin()
        .read_line(&mut input)
        .expect("hellos");

    let num: i32  =  input.trim().parse().expect("not a number");
    match  num.cmp(&hard_coded_num){
        Ordering::Less => print_result("Number is less"),
        Ordering::Equal => print_result("Number is Equal"),
        Ordering::Greater => print_result("Number is More"),
    }


}

fn print_result(str: &str){
    println!("{str}")
}
