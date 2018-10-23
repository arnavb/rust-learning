use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    let length: u32 = read_value("Enter the length of the rectangle: ",
        "Please enter a valid length!")
        .expect("An IO error occurred");
    
    let width: u32 = read_value("Enter the width of the rectangle: ",
        "Please enter a valid width!")
        .expect("An IO error occurred");
    
    let rectangle = Rectangle {
        length,
        width
    };
    
    println!("The area of the rectangle is {} units².", rectangle.area());
    
    println!("The perimeter of the rectangle is {} units.", rectangle.perimeter());
}

struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    
    fn perimeter(&self) -> u32 {
        2 * self.length + 2 * self.width
    }
}

fn read_value<T: FromStr>(prompt: &str, error_message: &str) -> Result<T, io::Error> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut result = String::new();
        
        io::stdin().read_line(&mut result)?;
        
        match result.trim().parse() {
            Ok(value) => return Ok(value),
            Err(_) => println!("{}", error_message)
        }
    }
}
