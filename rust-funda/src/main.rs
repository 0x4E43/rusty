fn main() {
    let mut x = 5; //once it is mutable by declaring as mut value can be reassigned
    println!("x is {x}");
    x = 6 ; //x can be assigned as every variable is mutable defined explicitly
    println!("x is {x}"); 


    const HOURS_IN_SECOND :i32 = 60*60; //constants cant be mutable
    let HOURS_IN_SECONDS = 40; // will throw warning as only constants should be upparcase
    //and variable should be lowercase
    println!("CONSTANAT: {HOURS_IN_SECOND}")
}
