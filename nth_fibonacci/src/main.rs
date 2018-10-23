use std::io::{self, Write};

fn main() {
    let input: u64 = loop {
        print!("Enter a positive integer: ");
        io::stdout().flush().expect("Unable to flush stdout!");

        let mut result = String::new();
    
        io::stdin().read_line(&mut result).expect("Unable to read input from stdin!");
        
        match result.trim().parse() {
            Ok(number) => break number,
            Err(_) => println!("The value you entered was not a valid positive integer!")
        }   
    };
    
    println!("Fibonacci #{} = {}", input, nth_fibonacci(input));
}

fn nth_fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    
    let mut previous2;
    let mut previous1 = 0;
    let mut current = 1;
    
    for _i in 0..(n - 1) {
        previous2 = previous1;
        previous1 = current;
        current = previous1 + previous2;
    }
    current
}
