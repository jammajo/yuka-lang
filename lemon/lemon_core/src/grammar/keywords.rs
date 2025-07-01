use serde::{Serialize, Deserialize};

/// Enum que representa todas las palabras clave (`keywords`) reservadas del lenguaje Yuka.
///
/// Estas palabras no pueden ser utilizadas como identificadores, ya que tienen un significado
/// especial para el analizador sintáctico y el compilador. Se utilizan para declarar control
/// de flujo, funciones, UI, comportamiento, entre otros.
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Keyword {
    // Control de flujo
    If,
    Elif,
    Else,
    Do,
    While,
    For,
    When,
    In,

    // Manejo de errores
    Try,
    Fail,

    // Estructura y control de ejecución
    End,
    Break,
    Continue,
    Return,
    Exit,

    // Declaraciones
    Let,
    Fn,
    From,
    As,
    None,

    // Funcionalidad adicional
    Back,
    Send,
    Del,
    Run,
    Log,
    Box,
    Wrap,

    // Booleanos
    True,
    False,

    // Palabras clave semánticas (ej. modificadores)
    Sure,
    Stay,
    Fast,
    Flat,
    Old,

    // Entrada/salida y herramientas
    Print,
    Ask,
    Read,
    Save,

    // Palabras clave relacionadas con la interfaz gráfica
    Ui,
    Btn,
    Txt,
    Img,
    Vid,
    Canvas,
    Div,
    Click,
    Window,
    Tick,
    Wait,
}

impl Keyword {
    /// Convierte una cadena (`&str`) a un valor `Keyword` si coincide con alguna palabra clave válida.
    ///
    /// La conversión es **insensible a mayúsculas** (`case-insensitive`), por lo que `"IF"` y `"if"`
    /// se tratan como equivalentes.
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Keyword::from_str("while"), Some(Keyword::While));
    /// assert_eq!(Keyword::from_str("WHILE"), Some(Keyword::While));
    /// assert_eq!(Keyword::from_str("xyz"), None);
    /// ```
    ///
    /// # Parámetros
    /// - `s`: La cadena a convertir.
    ///
    /// # Retorna
    /// `Some(Keyword)` si coincide con una palabra clave válida, o `None` en caso contrario.
    pub fn from_str(s: &str) -> Option<Keyword> {
        match s.to_lowercase().as_str() {
            // Control de flujo
            "if" => Some(Keyword::If),
            "elif" => Some(Keyword::Elif),
            "else" => Some(Keyword::Else),
            "do" => Some(Keyword::Do),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "when" => Some(Keyword::When),
            "in" => Some(Keyword::In),

            // Manejo de errores
            "try" => Some(Keyword::Try),
            "fail" => Some(Keyword::Fail),

            // Estructura y control de ejecución
            "end" => Some(Keyword::End),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "return" => Some(Keyword::Return),
            "exit" => Some(Keyword::Exit),

            // Declaraciones
            "let" => Some(Keyword::Let),
            "fn" => Some(Keyword::Fn),
            "from" => Some(Keyword::From),
            "as" => Some(Keyword::As),
            "none" => Some(Keyword::None),

            // Funcionalidad adicional
            "back" => Some(Keyword::Back),
            "send" => Some(Keyword::Send),
            "del" => Some(Keyword::Del),
            "run" => Some(Keyword::Run),
            "log" => Some(Keyword::Log),
            "box" => Some(Keyword::Box),
            "wrap" => Some(Keyword::Wrap),

            // Booleanos
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),

            // Palabras semánticas
            "sure" => Some(Keyword::Sure),
            "stay" => Some(Keyword::Stay),
            "fast" => Some(Keyword::Fast),
            "flat" => Some(Keyword::Flat),
            "old" => Some(Keyword::Old),

            // Entrada/salida y herramientas
            "print" => Some(Keyword::Print),
            "ask" => Some(Keyword::Ask),
            "read" => Some(Keyword::Read),
            "save" => Some(Keyword::Save),

            // Interfaz gráfica
            "ui" => Some(Keyword::Ui),
            "btn" => Some(Keyword::Btn),
            "txt" => Some(Keyword::Txt),
            "img" => Some(Keyword::Img),
            "vid" => Some(Keyword::Vid),
            "canvas" => Some(Keyword::Canvas),
            "div" => Some(Keyword::Div),
            "click" => Some(Keyword::Click),
            "window" => Some(Keyword::Window),
            "tick" => Some(Keyword::Tick),
            "wait" => Some(Keyword::Wait),

            // No coincide
            _ => None,
        }
    }
}
