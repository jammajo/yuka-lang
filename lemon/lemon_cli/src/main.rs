use std::env;
use std::fs;
use lemon_core::interpret;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Uso: yuka <archivo.yuka>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let input = fs::read_to_string(filename)
        .expect("No se pudo leer el archivo de entrada");

    match interpret(&input) {
        Ok(result) => {
            println!("{:?}", result); // Aquí puedes hacer `.trim()` si es necesario
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}
