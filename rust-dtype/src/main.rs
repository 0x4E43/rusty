fn main() {
    //below will give integer overflow as i8 can have maximum 255
    let guess :i16 = "300".parse().expect("not a number"); //type need to specified while parsing
    println!("Hello, world! {guess}");


    //floating points
    let float :f32= 3.0;
    println!("Float value: {float}");

    let x:f32 = 5.0;
    let y:f32 = 2.0;
    let res: f32 = x/y; 
    println!("Division: {res}");

    //boolean
    let b: bool = false;
    println!("Boolean : {b}");

    //tuples
    let tup :(i32, i32, i32 ) = (10, 11, 12);
    // println!("Tuples: {tup}") //cant be done
    let (x, y, z )= tup;
    println!("X= {x} Y= {y} Z={z}")


    
}
