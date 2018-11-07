mod mod_in_another_file;
mod submodule;

mod mod_inside_file {
    pub fn say_hello_to(something: &str) -> String {
        return format!("Hello, {} from mod_inside_file!", something);
    }
}

fn main() {
    println!("{}", mod_inside_file::say_hello_to("World"));
    println!("{}", mod_in_another_file::say_hello_to("World"));
    println!("{}", submodule::say_hello_to("World"));
    println!("{}", submodule::even_more_nested::say_hello_to("World"));
}
