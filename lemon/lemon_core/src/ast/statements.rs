use crate::ast::expressions::Expression;

/// Representa una instrucción completa del lenguaje Yuka.
///
/// A diferencia de `Expression`, los `Statement` no siempre producen un valor.
/// Su propósito es ejecutar acciones, como declarar variables, controlar flujo,
/// agrupar instrucciones, entre otros.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Declaración de variable con valor obligatorio.
    ///
    /// Ejemplo: `let x = 5;`
    Let {
        /// Nombre de la variable.
        name: String,
        /// Valor que se le asigna al declararla.
        value: Expression,
    },

    /// Declaración de función, con o sin nombre.
    ///
    /// Ejemplo: `fun greet(name) { ... }`
    Function {
        /// Nombre de la función (opcional si es anónima).
        name: Option<String>,
        /// Lista de nombres de parámetros.
        params: Vec<String>,
        /// Bloque de instrucciones que conforman el cuerpo de la función.
        body: Vec<Statement>,
    },

    /// Instrucción condicional `if`, con rama `else` opcional.
    ///
    /// Ejemplo: `if (cond) { ... } else { ... }`
    If {
        /// Condición a evaluar.
        condition: Expression,
        /// Rama ejecutada si la condición es verdadera.
        then_branch: Box<Statement>,
        /// Rama ejecutada si la condición es falsa (opcional).
        else_branch: Option<Box<Statement>>,
    },

    /// Bucle `while`, que repite mientras la condición se mantenga verdadera.
    ///
    /// Ejemplo: `while (cond) { ... }`
    While {
        /// Condición booleana de control.
        condition: Expression,
        /// Instrucciones que se repiten mientras la condición sea verdadera.
        body: Box<Statement>,
    },

    /// Bucle `do-while`, ejecuta primero el cuerpo y luego evalúa la condición.
    ///
    /// Ejemplo: `do { ... } while (cond);`
    DoWhile {
        /// Instrucciones que se ejecutan al menos una vez.
        body: Box<Statement>,
        /// Condición evaluada al final de cada iteración.
        condition: Expression,
    },

    /// Bucle estilo C: `for (init; condition; increment) { ... }`
    ///
    /// Ejemplo:
    /// ```
    /// for (let i = 0; i < 10; i = i + 1) { ... }
    /// ```
    ForCStyle {
        /// Inicialización de la variable del bucle.
        init: Box<Statement>,
        /// Condición de continuación.
        condition: Expression,
        /// Expresión ejecutada al final de cada iteración.
        increment: Expression,
        /// Instrucciones que se repiten mientras la condición sea verdadera.
        body: Box<Statement>,
    },

    /// Bucle estilo Python: `for var in iterable { ... }`
    ///
    /// Ejemplo: `for item in list { ... }`
    ForIn {
        /// Nombre de la variable iteradora.
        variable: String,
        /// Expresión que produce la secuencia a iterar.
        iterable: Expression,
        /// Instrucciones del cuerpo del bucle.
        body: Box<Statement>,
    },

    /// Instrucción `return`, para salir de una función con o sin valor.
    ///
    /// Ejemplo: `return;` o `return x + 2;`
    Return(Option<Expression>),

    /// Una expresión utilizada como instrucción.
    ///
    /// Ejemplo: `call();` o `x + 2;`
    Expr(Expression),

    /// Bloque de instrucciones agrupadas entre llaves `{ ... }`.
    ///
    /// Ejemplo:
    /// ```
    /// {
    ///     let x = 1;
    ///     print(x);
    /// }
    /// ```
    Block(Vec<Statement>),

    /// Finaliza la ejecución de un bucle o estructura repetitiva.
    ///
    /// Equivalente a `break;`
    Break,

    /// Finaliza una tarea, hilo o ciclo infinito de forma explícita.
    ///
    /// Puede usarse como instrucción semántica para marcar el fin de algo.
    End,

    /// Salta directamente a la siguiente iteración del bucle actual.
    ///
    /// Equivalente a `continue;`
    Continue,

    /// Expresión usada como instrucción (forma alternativa a `Expr`).
    ///
    /// Permite usar expresiones solas como declaraciones válidas.
    Expression(Expression),

    /// Declaración de variable con o sin inicialización.
    ///
    /// Ejemplo: `let x;` o `let x = 42;`
    Variable {
        /// Nombre de la variable declarada.
        name: String,
        /// Valor opcional con el que se inicializa.
        initializer: Option<Expression>,
    },

    /// Llamada a una función interna del lenguaje o del runtime.
    ///
    /// Ejemplo: `@print("Hola")`
    BuiltinCall(String, Expression),
}
