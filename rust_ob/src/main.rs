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
}

