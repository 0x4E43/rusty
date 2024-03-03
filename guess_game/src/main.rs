use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    //generate a random number between 0-100
    let guess_num = rand::thread_rng().gen_range(1..=100);
    println!("Hello, world! {guess_num}");
    guess_function(guess_num);


}
fn print_result(str: &str){
    println!("{str}")
}

fn guess_function(guess_num :i32){
    loop {
        
        let mut input = String::new();
        // take input
        io::stdin()
            .read_line(&mut input)
            .expect("hellos");
    
        let num: i32  =  match  input.trim().parse(){
            Ok(n) => n,
            Err(_) => continue,
        };
        
        match  num.cmp(&guess_num){
            Ordering::Less => print_result("Number is less"),
            Ordering::Equal => {
                print_result("You win!");
                break;
            },
            Ordering::Greater => print_result("Number is More"),
        }
    }
}


