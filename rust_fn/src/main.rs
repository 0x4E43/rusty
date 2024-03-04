fn main() {
    println!("Hello, world!");

    let x = {
        let y = 5;
        y+100
    };

    println!("X is {x}");

    let z = {let u = 6; u};
    println!("Z : {z}");
    test_function();
    println!("value from return_test() {}", return_test());
}

fn return_test() -> i32{
    5 //default return if semicolon is not present
}

fn test_function(){
    println!("Test function called");
    let  ll = String::from("hello");
    let (u, v) = tt(ll);
    println!("After tt U: {u} V: {v}") ;
}

fn tt(st :String) -> (String, String) {
    let st = st+ " hello";
    return  (st, String::from("nimai"));
}