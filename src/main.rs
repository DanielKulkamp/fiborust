use std::io;

fn main() {
    println!("Enter n for the nth Fibonacci number:");
    let mut nstr = String::new();
    let mut n : u64;
    loop {

        io::stdin().read_line(&mut nstr)
            .expect("Something went wrong");

        n = match nstr.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid positive integer!");
                nstr = String::new();
                continue;
            }
        };
        break;
    };
    if n < 2 {
        println!("1");
    } else {
        let mut n1 = 1u64;
        let mut n2 = 1u64;
        while n>1 {
            n = n-1;
            n2 = n2+n1;
            n1 = n2-n1;                
        }
        println!("{}",n2);
    }
}
