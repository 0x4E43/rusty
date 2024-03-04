fn main() {
    let mut x = 5; //once it is mutable by declaring as mut value can be reassigned
    println!("x is {x}");
    x = 6 ; //x can be assigned as every variable is mutable defined explicitly
    println!("x is {x}"); 


    const HOURS_IN_SECOND :i32 = 60*60; //constants cant be mutable
    let hour_in_second  = 60*60; // will throw warning as only constants should be upparcase
    //and variable should be lowercase
    println!("CONSTANAT: {HOURS_IN_SECOND}");
    println!("NOT CONSTANAT: {hour_in_second}");

    //SHADOWING
    let var = 10;
    println!("var before : {var}");
    let var = var + 5;
    println!("var before scope : {var}");
    {
        let var = var + 10;
        println!("Local scope var is {var}")
    }//can be made inner using braces
    println!("VAR is : {var}");

}
