use std::io::stdin;
pub fn input(msg: &str) -> String {
    let mut user_input = String::new();
    println!("{}", msg);
    stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("Error from reading stdin: {}", e);
        0
    });

    user_input
}