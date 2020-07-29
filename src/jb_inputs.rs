use std::io::stdin;
/// prints message to terminal, and returns formatted user's input
/// e.g parsing out \r and \n,
/// Inspired by Python's input function
pub fn input(msg: &str) -> String {
    let mut user_input = String::new();
    println!("{}", msg);
    stdin().read_line(&mut user_input).unwrap_or_else(|e| {
        println!("Error from reading stdin: {}", e);
        0
    });

    user_input.replace("\r", "").replace("\n", "")
}

