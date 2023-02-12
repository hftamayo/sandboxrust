use std::io;
fn main() {
    println!("To quit please type exit");
    loop{
        println!("type any positive number");
        let mut number = String::new();
        io::stdin()
        .read_line(& mut number)
        .expect("failed to read your input");
        if number.trim() == "exit"{
            break;
        }
        let number: i32 = match number.trim().parse(){
            Ok(number) => number,
            Err(_) => continue,
        };
        println!("factorial ({}) => {}", number, factorial(number));
    }
}

fn factorial(n: i32) -> i32 {
    if n < 2 {
        return 1;
    } else {
        return n * factorial(n - 1) 
    }
}
