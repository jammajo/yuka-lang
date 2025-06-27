# Lista de Keywords del Lenguaje de Programación Personalizado

## 🔢 Control de Flujo
| Palabra clave | Uso                                                           |
| ------------- | ------------------------------------------------------------- |
| `if`          | Iniciar una condicional.                                      |
| `elif`        | Condicional alternativa (else if).                            |
| `else`        | Caso contrario.                                               |
| `loop`        | Bucle que verifica la condición antes de ejecutar.            |
| `do`          | Inicia un bloque que se ejecuta antes de verificar condición. |
| `for`         | Bucle tradicional estilo C/C++.                               |
| `try`         | Inicia un bloque de manejo de errores.                        |
| `fail`        | Captura errores del bloque `try`.                             |
| `end`         | Finaliza bloques (`if`, `loop`, `fn`, etc.).                  |

## 🔹 Declaración y Variables
| Palabra clave | Uso                                                                                                                           |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `let`         | Declarar una variable (tipado opcional).                                                                                      |
| `from`        | Importar módulos o librerías externas.                                                                                        |
| `back`        | Retornar un valor desde una función.                                                                                          |
| `send`        | Declarar una variable como pública para acceso desde fuera del bloque o módulo. Todas las variables son privadas por defecto. |
| `del`         | Eliminar una variable o recurso.                                                                                              |

## 💻 Funciones
| Palabra clave | Uso                                         |
| ------------- | ------------------------------------------- |
| `fn`          | Declarar una función.                       |
| `call`        | Llamar una función almacenada como variable.|

## 🏷️ Atributos para Variables y Funciones
| Atributo  | Aplicable a      | Uso                                                                 |
|-----------|------------------|----------------------------------------------------------------------|
| `sure`    | Variables         | Hace que una variable no pueda ser modificada luego de creada.       |
| `stay`    | Variables, Funcs | Define valores o funciones asociados al módulo, no a instancias.     |
| `fast`    | Funciones         | Marca una función como asíncrona (para ejecución no bloqueante).     |
| `flat`    | Funciones         | Sugiere que la función sea evaluada directamente en el lugar.        |
| `old`     | Variables, Funcs | Marca elementos como obsoletos para advertencia en tiempo de uso.    |

## 📃 Entrada/Salida
| Palabra clave | Uso                                       |
| ------------- | ----------------------------------------- |
| `print`        | Mostrar texto u otros valores en consola. |
| `ask`         | Recibir datos desde el usuario por consola. |
| `read`        | Leer un archivo externo.                  |
| `save`        | Escribir datos en un archivo.             |

## 🎨 Gráficos / GUI
| Palabra clave | Uso                                           |
| ------------- | --------------------------------------------- |
| `btn`         | Crear un botón.                               |
| `txt`         | Mostrar texto en la interfaz.                 |
| `box`         | Campo de entrada de texto.                    |
| `img`         | Mostrar una imagen.                           |
| `vid`         | Mostrar un video embebido.                    |
| `wrap`        | Componente de agrupación visual, tipo `div`.  |
| `when`        | Evento de clic para componentes interactivos. |
| `ui`          | Activar el sistema gráfico.                   |

## ⚙️ Otros
| Palabra clave | Uso                                      |
| ------------- | ---------------------------------------- |
| `#`           | Iniciar un comentario.                   |
| `tick`        | Ejecutar ciclo de motor (main loop).     |
| `wait`        | Esperar cierta cantidad de tiempo.       |
| `as`          | Crear alias para importaciones.          |
| `type`        | Definir un tipo de dato personalizado.   |
| `self`        | Referencia al objeto actual (OOP futura).|
| `log`         | Mostrar información de depuración.       |
| `run`         | Ejecutar scripts o comandos externos.     |
| `exit`        | Terminar la ejecución del programa.      |
