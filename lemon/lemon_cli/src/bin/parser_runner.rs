use std::fs;

fn main() {
    let dir = "tests/parser";

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Solo archivos con extensión .yk
        if path.extension().and_then(|s| s.to_str()) != Some("yk") {
            continue;
        }

        let name = path.file_name().unwrap().to_str().unwrap();
        let source = fs::read_to_string(&path).unwrap();

        match lemon_core::interpret(&source) {
            Ok(statements) => {
                println!("[PASS] {} → AST generado correctamente\n", name);
                // Aquí puedes imprimir el AST si quieres
                // println!("{:#?}", statements);
            }

            Err(e) => {
                println!("[FAIL] {} → Error:\n{}\n", name, e);
            }
        }
    }
}
