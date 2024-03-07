//for ownership and borrowing

fn main() {
    let x =10;
    println!("Hello, world!");
    {
        let x = 100;
        println!("X is {x}");
    }
    println!("X is {x}");


    let mut s = String::from("hello");
    s.push_str(" string");
    println!("{}", s);


    let s1 = String::from("hello");
    let s2 = s1; 
    let s3 = s2.clone();  //It can work on String, what about primitive type 
    println!("S2- {}", s2);
    println!("S3- {}", s3);

    println!("{}, world!", s2);

    let x = 5;
    let y = x; // x is not dropped here, why?
    println!("{} {}", x,y);

    let mut st =  String::from("Nimai");
    test(&st);
    println!("After test {st}");
    test_print(&mut st); //as we are playing with refence scope will be there
    println!("After change: {st} Dangle: {}", no_dangle())
}

fn test(st : &str)-> &str{
    println!("String {}", st);
    st //this will return st without return statement
}

//test print function
fn test_print(st : &mut String){
    println!("ST: {}", st);
    st.push_str("Charan")
}

//dangling reference
fn no_dangle() -> String{
    let s = String::from("Hello Rust🦀");
    s
}

// fn dangle() -> &String{
//     &String::from("value")
// }