use std::io;
fn main() {
    println!("To quit please type exit");
    loop{
        println!("Type any positive number");
        let mut number = String::new();
        io::stdin()
        .read_line(& mut number)
        .expect("failed to read your input");
        if number.trim() == "exit"{
            break;
        }
        let number: i32 = match number.trim()
        .parse(){
            Ok(number) => number,
            Err(_) => continue,
            };
        println!("fibonacci ({}) => {}", number, fib(number));
    }
}

fn fib (n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } fib(n - 1) + fib(n - 2)
}
