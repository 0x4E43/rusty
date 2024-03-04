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
}
