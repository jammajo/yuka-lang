# Ejemplos de Uso y Funcionalidad de Keywords

## 🔢 Control de Flujo
```tu_lang
if x > 10
  show "Mayor que 10"
elif x == 10
  show "Igual a 10"
else
  show "Menor que 10"
end

loop i < 5
  show i
  i = i + 1
end

do
  show "Se ejecuta al menos una vez"
  i = i + 1
loop i < 5
end

for i = 0; i < 3; i = i + 1
  show i
end

try
  call dividir()
fail e
  show "Error: " + e
end
```

## 🔹 Declaración y Variables
```tu_lang
let edad = 25
let nombre: string = "Ana"
send user_id = 12345  # pública

del edad
```

## 💻 Funciones
```tu_lang
fn saludar(nombre)
  show "Hola, " + nombre
end

ask "¿Cómo te llamas?" as nombre
call saludar(nombre)
```

## 🏷️ Atributos para Variables y Funciones
```tu_lang
let PI sure = 3.1416

fn log_in() fast
  show "Inicio de sesión..."
end

old fn vieja()
  show "Esto ya no se usa"
end
```

## 📃 Entrada/Salida
```tu_lang
show "Hola mundo"
let texto = read "archivo.txt"
save "resultado.txt" texto
```

## 🎨 Gráficos / GUI
```tu_lang
ui

btn text="Click me" when=mi_evento

fn mi_evento()
  show "¡Botón presionado!"
end

wrap
  txt text="Bienvenido"
  box id="entrada"
end
```

## ⚙️ Otros
```tu_lang
# Esto es un comentario

wait 1000  # milisegundos

tick
  show "Actualizando..."
end

from ui as interfaz

type Punto
  let x
  let y
end

run "limpiar_cache.sh"
exit
```