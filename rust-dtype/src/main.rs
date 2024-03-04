fn main() {
    let guess :i8 = "42".parse().expect("not a number"); //type need to specified while parsing
    println!("Hello, world! {guess}");
}
