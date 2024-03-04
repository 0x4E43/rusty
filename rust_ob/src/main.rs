//for ownership and borrowing

fn main() {
    let x =10;
    println!("Hello, world!");
    {
        let x = 100;
        println!("X is {x}");
    }
    println!("X is {x}")
}

