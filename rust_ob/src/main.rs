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

    println!("{}, world!", s2);
}

