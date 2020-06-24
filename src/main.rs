use std::io;

fn main() {
    println!("Enter n for the nth Fibonacci number:");

    let mut n = String::new();
    io::stdin.read_line(&mut n);
    println!("{}",n);
        
}
