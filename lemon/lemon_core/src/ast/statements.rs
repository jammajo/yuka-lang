use crate::ast::expressions::Expression;

/// Representa una instrucción completa del lenguaje Yuka.
/// A diferencia de `Expression`, los `Statement` son acciones o bloques que no necesariamente devuelven un valor.
#[derive(Debug, Clone)]
pub enum Statement {
    /// Declaración de variable: let x = expr;
    Let {
        name: String,        // Nombre de la variable
        value: Expression,   // Valor asignado
    },

    /// Declaración de función: fun name(params) { body }
    Function {
        name: String,            // Nombre de la función
        params: Vec<String>,     // Lista de parámetros (identificadores)
        body: Vec<Statement>,    // Cuerpo de la función (bloque de instrucciones)
    },

    /// Condicional if: if (condition) { then_branch } else { else_branch }
    If {
        condition: Expression,           // Condición booleana
        then_branch: Vec<Statement>,     // Bloque si la condición es verdadera
        else_branch: Option<Vec<Statement>>, // Bloque si es falsa (opcional)
    },

    /// Bucle while: while (condition) { body }
    While {
        condition: Expression,       // Condición para continuar el bucle
        body: Vec<Statement>,        // Instrucciones del cuerpo del bucle
    },

    /// Bucle do-while: do { body } while (condition);
    /// Ejecuta el cuerpo al menos una vez antes de verificar la condición.
    DoWhile {
        body: Vec<Statement>,        // Instrucciones del cuerpo
        condition: Expression,       // Condición para continuar
    },

    /// Instrucción de retorno desde una función: return expr;
    /// Puede no incluir expresión (return sin valor).
    Return(Option<Expression>),

    /// Una expresión suelta como instrucción: x + 1;, callFn();, etc.
    Expr(Expression),

    /// Bloque de instrucciones agrupadas con llaves: { ... }
    Block(Vec<Statement>),
}
