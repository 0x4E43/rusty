fn main() {
    //below will give integer overflow as i8 can have maximum 255
    let guess :i8 = "300".parse().expect("not a number"); //type need to specified while parsing
    println!("Hello, world! {guess}");
}
