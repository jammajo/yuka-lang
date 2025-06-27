use lemon_core::interpret;

fn main() {
    let input = "print(add 2,3)";
    match interpret(input) {
        Ok(result) => println!("Tokens: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
