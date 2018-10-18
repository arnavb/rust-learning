use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    println!("Welcome to the temperature converter!");
    
    println!("Pick a conversion:");
    println!("[1] Fahrenheit to Celsius");
    println!("[2] Celsius to Fahrenheit");
    
    let choice: u32 = loop {
        let value = read_value_from_input("> ", "Please enter a valid integer!");
        
        if value == 1 || value == 2 {
            break value;
        }
        println!("Please enter a valid choice (0 or 1)!");
    };

    if choice == 1 {
        let temperature: f64 = read_value_from_input("Enter the temperature to convert: ",
            "Please enter a valid floating point variable!");
        
        println!("{:.2} °F is {:.2} °C.", temperature, (temperature - 32f64) * 5f64 / 9f64);

    } else if choice == 2 {
        let temperature: f64 = read_value_from_input("Enter the temperature to convert: ",
            "Please enter a valid floating point variable!");
        
        println!("{:.2} °C is {:.2} °F.", temperature, temperature * 9f64 / 5f64 + 32f64);

    } else {
        println!("{} was not a valid choice!", choice);
    }
}

fn read_value_from_input<T: FromStr>(prompt: &str, error_message: &str) -> T {
    let result: T = loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Unable to flush STDOUT!");
        
        let mut input_value = String::new();
        
        io::stdin().read_line(&mut input_value)
            .expect(error_message);
        
        match input_value.trim().parse() {
            Ok(value) => break value,
            Err(_) => {
                println!("{}", error_message);
                continue;
            }
        }
    };
    result
}
