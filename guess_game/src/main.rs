use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;


fn main() {
    //generate a random number between 0-100
    println!("{}", "Enter a number to guess :".magenta().bold());
    let guess_num = rand::thread_rng().gen_range(1..=100);
    guess_function(guess_num);


}
fn print_result(str: &str, num :i32){
    println!("{} {num} is {str}", "SORRY!".blue())
}

fn guess_function(guess_num :i32){
    loop {
        let mut input = String::new();
        
        // take input
        match io::stdin()
            .read_line(&mut input) {
                Ok( input) => input,
                Err(_) => {
                    println!("something went wrong");
                    continue
                },
            };
    
        let num: i32  =  match  input.trim().parse(){
            Ok(n) => n,
            Err(_) => {
                println!("{}", "Please enter a number".red());
                continue},
        };
        match  num.cmp(&guess_num){
            Ordering::Less => print_result("Less", num),
            Ordering::Equal => {
                println!("{}", "Matched, You Won!".green());
                break;
            },
            Ordering::Greater => print_result("More", num),
        }
    }
}

