use std::io::stdin;
fn input(msg: &str) -> String {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("Error from reading stdin: {}", e);
        0
    });

    user_input
}
fn main() {
    
    
}