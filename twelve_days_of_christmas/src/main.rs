use std::io::{self, Write};

fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
                "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let song_lines = ["Two turtle doves", "Three french hens", "Four calling birds",
                      "Five golden rings", "Six geese-a-laying", "seven swans-a-swimming",
                      "Eight maids a-milking","Nine ladies dancing","Ten lords a-leaping",
                      "Eleven pipers piping", "Twelve drummers drumming"];
    
    for i in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", days[i]);
        
        for j in (0..i).rev() {
            print!("{},", song_lines[j]);
            io::stdout().flush().expect("Unable to flush STDOUT!");
            
            println!("{}", if j == 0 { " and" } else { "" });
        }
        
        println!("A partridge in a pear tree\n");
    }
}
