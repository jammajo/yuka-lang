use crate::ast::expressions::Expression;

/// Representa una instrucción completa del lenguaje Yuka.
/// A diferencia de `Expression`, los `Statement` son acciones que ocurren por sí mismas,
/// y no necesariamente producen un valor. Ejemplos incluyen declaraciones, bucles, bloques, etc.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Declaración de variable con valor obligatorio (forma alternativa a `Variable`)
    /// Ejemplo: `let x = 5;`
    Let {
        name: String,        // Nombre de la variable
        value: Expression,   // Valor que se asigna al declararla
    },

    /// Declaración de función, con o sin nombre.
    /// Ejemplo: `fun greet(name) { ... }`
    Function {
        name: Option<String>,      // Nombre de la función (puede ser anónima)
        params: Vec<String>,       // Lista de parámetros (identificadores)
        body: Vec<Statement>,      // Bloque de instrucciones que componen la función
    },

    /// Condicional `if`, con rama `else` opcional.
    /// Ejemplo: `if (cond) { ... } else { ... }`
    If {
        condition: Expression,                // Condición que se evalúa
        then_branch: Box<Statement>,          // Bloque ejecutado si la condición es verdadera
        else_branch: Option<Box<Statement>>,  // Bloque ejecutado si es falsa (opcional)
    },

    /// Bucle `while`, que repite mientras la condición sea verdadera.
    /// Ejemplo: `while (cond) { ... }`
    While {
        condition: Expression,     // Condición booleana
        body: Box<Statement>,      // Cuerpo del bucle (bloque a repetir)
    },

    /// Bucle `do-while`, que ejecuta el cuerpo al menos una vez.
    /// Ejemplo: `do { ... } while (cond);`
    DoWhile {
        body: Box<Statement>,      // Cuerpo del bucle
        condition: Expression,     // Condición evaluada después de la ejecución
    },

    /// Bucle estilo C: `for (init; condition; increment) { body }`
    ForCStyle {
        init: Box<Statement>,      // Declaración o asignación inicial
        condition: Expression,     // Condición que determina si continúa
        increment: Expression,     // Expresión ejecutada después de cada iteración
        body: Box<Statement>,      // Cuerpo del bucle
    },

    /// Bucle estilo Python: `for var in iterable { body }`
    ForIn {
        variable: String,          // Variable iteradora (ej. `i`)
        iterable: Expression,      // Expresión que produce una secuencia iterable
        body: Box<Statement>,      // Cuerpo del bucle
    },

    /// Retorno desde una función: `return expr;`
    /// Si no hay valor, representa `return;` (con `None`)
    Return(Option<Expression>),

    /// Expresión usada como instrucción: `call();` o `a + b;`
    Expr(Expression),

    /// Bloque de múltiples instrucciones agrupadas: `{ stmt1; stmt2; ... }`
    Block(Vec<Statement>),

    /// Interrupción de bucle o switch: `break;`
    Break,

    /// Marca de fin o finalización explícita (puede usarse para ciclos infinitos, tareas, etc.)
    End,

    /// Continúa con la siguiente iteración de un bucle: `continue;`
    Continue,

    /// Alias para usar expresiones sueltas como statements (útil para diseño flexible)
    Expression(Expression),

    /// Declaración de variable con inicialización opcional.
    /// Ejemplo: `let x;` o `let x = 42;`
    Variable {
        name: String,                 // Nombre de la variable
        initializer: Option<Expression>, // Valor opcional (puede ser None)
    },
}
