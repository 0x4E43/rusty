fn main() {
    let x = 0;
    if x <4 { //parentesis around if block will through error
        println!("Hello, world!");
    }else{
        println!("Hello, India!")
    }
    let x:bool = true;
    if x{
        println!("Helloooooooo");
    }

    let h = 100;

    let condition = true;
    let number = if condition { h*5 } else { 6 };

    println!("The value of number is: {number}");



    let mut count = 0;
    'counting_up: loop { // 'counting_up is loop label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
            loop {
                if remaining==9{

                    break 'counting_up;
                }
            }
        }

        count += 1;
    }
    println!("End count = {count}");



    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


    let a = [1, 45, 555, 55, 786, 773];
    println!("FOR LOOP");
    for el in a{
        println!("Element from a {el}")
    }

    println!("WHILE LOOP");
    let mut index = 0;
    while index < a.len(){
        println!("Element from a {}", a[index]);
        index = index+1;

    }


}
