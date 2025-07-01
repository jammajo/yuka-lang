/// Representa cualquier expresión que produce un valor en el lenguaje Yuka.
/// 
/// Las expresiones incluyen literales, operaciones, llamadas a funciones,
/// acceso a propiedades, entre otros. Se usan en contextos donde se espera
/// una evaluación que retorne un valor.
#[derive(Debug, Clone)]
pub enum Expression {
    /// Literal numérico, por ejemplo: `42`, `3.14`, etc.
    Number(f64),

    /// Literal de texto, como `"hola"` o `"Yuka"`.
    String(String),

    /// Valor booleano literal: `true` o `false`.
    Boolean(bool),

    /// Referencia a una variable previamente declarada.
    /// Por ejemplo: `x`, `nombre`, `counter`, etc.
    Variable(String),

    /// Expresión de operación binaria entre dos expresiones.
    /// Se utiliza para operaciones aritméticas, comparaciones y lógicas.
    /// Ejemplos: `a + b`, `x > 10`, `flag && condition`
    Binary {
        /// Lado izquierdo de la operación (puede ser cualquier expresión).
        left: Box<Expression>,
        /// Operador binario que define la operación.
        op: BinaryOp,
        /// Lado derecho de la operación.
        right: Box<Expression>,
    },

    /// Expresión de operación unaria sobre una sola expresión.
    /// Por ejemplo: `-x`, `!activo`
    Unary {
        /// Operador unario aplicado.
        op: UnaryOp,
        /// Expresión sobre la que se aplica el operador.
        expr: Box<Expression>,
    },

    /// Llamada a una función con uno o varios argumentos.
    /// Por ejemplo: `saludar("Juan")`
    Call {
        /// Expresión que representa la función a invocar.
        /// Puede ser un identificador simple o una propiedad.
        function: Box<Expression>,
        /// Lista de expresiones que se pasan como argumentos.
        args: Vec<Expression>,
    },

    /// Agrupación explícita de una expresión entre paréntesis.
    /// Utilizada para alterar el orden de evaluación.
    /// Ejemplo: `(1 + 2) * 3`
    Grouping(Box<Expression>),

    /// Asignación de un nuevo valor a una variable ya existente.
    /// Ejemplo: `x = 5`
    Assign {
        /// Nombre de la variable a la que se le asigna el nuevo valor.
        variable: String,
        /// Valor a asignar.
        value: Box<Expression>,
    },

    /// Acceso a una propiedad de un objeto.
    /// Ejemplo: `persona.nombre`
    Get {
        /// Expresión que representa el objeto contenedor.
        object: Box<Expression>,
        /// Nombre de la propiedad que se desea acceder.
        name: String,
    },

    /// Modificación de una propiedad de un objeto.
    /// Ejemplo: `persona.edad = 30`
    Set {
        /// Expresión que representa el objeto contenedor.
        object: Box<Expression>,
        /// Nombre de la propiedad a modificar.
        name: String,
        /// Nuevo valor que se le asigna a la propiedad.
        value: Box<Expression>,
    },

    /// Expresión condicional ternaria, con una condición y dos posibles resultados.
    /// Ejemplo: `es_admin ? "Sí" : "No"`
    Ternary {
        /// Condición a evaluar.
        condition: Box<Expression>,
        /// Resultado si la condición es verdadera.
        then_branch: Box<Expression>,
        /// Resultado si la condición es falsa.
        else_branch: Box<Expression>,
    },

    Literal(Literal),


    /// Valor nulo o indefinido. Representa la ausencia de un valor.
    /// Similar a `null` o `undefined` en otros lenguajes.
    None,
}

/// Enum que define todos los operadores binarios que soporta Yuka.
///
/// Estos operadores se usan en expresiones que combinan dos operandos.
#[derive(Debug, Clone)]
pub enum BinaryOp {
    /// Suma: `+`
    Add,
    /// Resta: `-`
    Sub,
    /// Multiplicación: `*`
    Mul,
    /// División: `/`
    Div,

    /// Igualdad: `==`
    Eq,
    /// Desigualdad: `!=`
    Neq,
    /// Mayor que: `>`
    Gt,
    /// Menor que: `<`
    Lt,
    /// Mayor o igual: `>=`
    Gte,
    /// Menor o igual: `<=`
    Lte,

    /// Conjunción lógica: `&&`
    And,
    /// Disyunción lógica: `||`
    Or,
}

/// Enum que define todos los operadores unarios disponibles.
///
/// Los operadores unarios actúan sobre una sola expresión.
#[derive(Debug, Clone)]
pub enum UnaryOp {
    /// Negación lógica: `!expr`
    Not,
    /// Negación aritmética (cambia el signo): `-expr`
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    None,
}

