use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    println!("Welcome to the temperature converter!");
    
    println!("Pick a conversion:");
    println!("[1] Fahrenheit to Celsius");
    println!("[2] Celsius to Fahrenheit");
    
    let choice: u32 = loop {
        let value = read_value("> ", "Please enter a valid positive integer!")
            .expect("An IO stream error occurred!");
        
        if value == 1 || value == 2 {
            break value;
        }
        println!("Please enter a valid choice (0 or 1)!");
    };

    let temperature: f64 = read_value("Enter the temperature to convert: ",
        "Please enter a valid floating point number!")
        .expect("An IO stream error occurred!");

    if choice == 1 {
        println!("{:.2} °F is {:.2} °C.", temperature, (temperature - 32f64) * 5f64 / 9f64);
    } else {
        println!("{:.2} °C is {:.2} °F.", temperature, temperature * 9f64 / 5f64 + 32f64);
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
