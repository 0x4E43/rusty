fn main() {
    println!("Hello, world!");
    test_function();
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