use lemon_core::interpret;

fn main() {
    let input = "print (add 2, 3)";

    match interpret(input) {
        Ok(result) => {
            for line in result.lines() {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }
}
