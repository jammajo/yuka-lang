use serde::{Serialize, Deserialize};

/// Enum que representa los diferentes tipos de comentarios reconocidos por el lenguaje Yuka.
///
/// Los comentarios pueden ser de una sola línea (`//`) o bloques delimitados por `/* */`.
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Comment {
    /// Comentario de una sola línea (inicia con `//`).
    LineComment,

    /// Inicio de un comentario en bloque (inicia con `/*`).
    BlockCommentOpen,

    /// Fin de un comentario en bloque (termina con `*/`).
    BlockCommentClose,
}

impl Comment {
    /// Intenta convertir una cadena en un tipo de comentario válido.
    ///
    /// Recibe una cadena como `"//"`, `"/*"` o `"*/"` y devuelve el tipo de comentario correspondiente.
    ///
    /// # Ejemplo
    /// ```
    /// assert_eq!(Comment::from_str("//"), Some(Comment::LineComment));
    /// ```
    ///
    /// Retorna `None` si la cadena no representa un delimitador de comentario reconocido.
    pub fn from_str(s: &str) -> Option<Comment> {
        match s {
            "//" => Some(Comment::LineComment),
            "/*" => Some(Comment::BlockCommentOpen),
            "*/" => Some(Comment::BlockCommentClose),
            _ => None, // No es un delimitador de comentario válido
        }
    }
}
