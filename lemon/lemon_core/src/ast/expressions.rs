/// Representa cualquier expresión que devuelve un valor en Yuka.
#[derive(Debug, Clone)]
pub enum Expression {
    /// Un número literal, como 42 o 3.14
    Number(f64),

    /// Una cadena de texto, como "hola"
    String(String),

    /// Un valor booleano: true o false
    Boolean(bool),

    /// Una variable, como x o nombre
    Variable(String),

    /// Una operación binaria: a + b, x > 10, etc.
    Binary {
        left: Box<Expression>,     // lado izquierdo de la operación
        op: BinaryOp,              // operador (suma, resta, etc.)
        right: Box<Expression>,    // lado derecho de la operación
    },

    /// Una operación unaria: -x, !flag
    Unary {
        op: UnaryOp,               // operador unario (!, -)
        expr: Box<Expression>,     // expresión sobre la que se aplica
    },

    /// Llamada a función: miFuncion(1, 2)
    Call {
        function: Box<Expression>, // función a invocar (puede ser variable o expresión)
        args: Vec<Expression>,     // lista de argumentos
    },

    /// Agrupación de una expresión con paréntesis: (1 + 2)
    Grouping(Box<Expression>),

    /// Asignación de valor a una variable existente: x = 5
    Assign {
        variable: String,          // nombre de la variable
        value: Box<Expression>,    // nuevo valor
    },

    /// Acceso a una propiedad: objeto.propiedad
    Get {
        object: Box<Expression>,   // expresión que representa al objeto
        name: String,              // nombre de la propiedad
    },

    /// Modificación de una propiedad: objeto.propiedad = valor
    Set {
        object: Box<Expression>,   // expresión del objeto
        name: String,              // propiedad a modificar
        value: Box<Expression>,    // nuevo valor de la propiedad
    },

    /// Expresión ternaria: cond ? a : b
    Ternary {
        condition: Box<Expression>,    // condición a evaluar
        then_branch: Box<Expression>,  // resultado si la condición es verdadera
        else_branch: Box<Expression>,  // resultado si es falsa
    },

    /// Representa un valor nulo o vacío (como null o undefined)
    None,
}

/// Representa los operadores binarios disponibles en Yuka.
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,  // +
    Sub,  // -
    Mul,  // *
    Div,  // /

    Eq,   // ==
    Neq,  // !=
    Gt,   // >
    Lt,   // <
    Gte,  // >=
    Lte,  // <=

    And,  // &&
    Or,   // ||
}

/// Representa los operadores unarios disponibles en Yuka.
#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,  // !
    Neg,  // - (negativo)
}
