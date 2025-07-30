# To-Do List en Rust 🦀

![Rust](https://img.shields.io/badge/Rust-1.80+-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

Bienvenido/a al proyecto **To-Do List en Rust**, un mini-proyecto de consola diseñado para aprender y practicar las características fundamentales del lenguaje de programación **Rust**. Este proyecto es ideal para principiantes que desean explorar conceptos como *ownership*, *borrowing*, *structs*, *enums*, *pattern matching* y más, mientras construyen una aplicación práctica y funcional.

## 🎯 Objetivo

El objetivo de este proyecto es crear una aplicación de consola que permita gestionar una lista de tareas (*To-Do List*). Los usuarios pueden agregar, listar, completar y eliminar tareas, además de guardarlas en un archivo JSON para persistencia. Este proyecto está diseñado para:

- **Aprender Rust**: Practicar conceptos clave del lenguaje de forma práctica.
- **Construir algo útil**: Crear una herramienta sencilla para gestionar tareas.
- **Escalabilidad**: Servir como base para añadir más funcionalidades y seguir aprendiendo.

## 📋 Características

- Agregar tareas con descripción y prioridad (baja, media, alta).
- Listar todas las tareas con su estado (pendiente o completada).
- Marcar tareas como completadas.
- Eliminar tareas por ID.
- Guardar y cargar tareas desde un archivo JSON (`tasks.json`).
- Interfaz de consola interactiva con un menú simple.

## 🚀 Instalación y Ejecución

### Prerrequisitos
- [Rust](https://www.rust-lang.org/tools/install) (versión 1.80 o superior) y Cargo instalados.
- Un editor de texto (recomendado: [VS Code](https://code.visualstudio.com/) con la extensión `rust-analyzer`).

### Pasos para Configurar
1. Clona este repositorio:
   ```bash
   git clone https://github.com/<tu-usuario>/todo_list.git
   cd todo_list
   ```

2. Compila y ejecuta el proyecto:
   ```bash
   cargo run
   ```

3. (Opcional) Verifica el código con pruebas y formato:
   ```bash
   cargo test
   cargo fmt
   ```

### Dependencias
El proyecto utiliza las siguientes crates (especificadas en `Cargo.toml`):
- `serde` y `serde_json`: Para serialización/deserialización de tareas en formato JSON.

## 🛠️ Criterios de Aceptación

Estos son los criterios que el programa debe cumplir para considerarse completo. Úsalos como una guía para desarrollar o verificar el proyecto:

1. **Inicialización del Programa**:
   - Al iniciar, carga las tareas desde `tasks.json` si existe.
   - Si no existe o está corrupto, crea una nueva lista vacía y muestra: "No se encontró un archivo de tareas, creando una nueva lista."

2. **Menú de Consola**:
   - Muestra un menú con opciones: 1) Agregar tarea, 2) Listar tareas, 3) Completar tarea, 4) Eliminar tarea, 5) Salir.
   - Acepta entradas numéricas (1-5) y muestra un mensaje de error para entradas inválidas.

3. **Agregar Tarea**:
   - Permite ingresar una descripción (texto libre).
   - Pide la prioridad (low, medium, high) y asigna `Low` por defecto si es inválida.
   - Asigna un ID único a cada tarea (contador incremental).
   - Agrega la tarea con estado `completed = false`.
   - Muestra: "Tarea agregada."

4. **Listar Tareas**:
   - Muestra todas las tareas con ID, descripción, prioridad y estado (completada o pendiente).
   - Si no hay tareas, muestra: "No hay tareas en la lista."
   - Formato sugerido: "ID: X | Descripción: XXX | Prioridad: XXX | Estado: XXX".

5. **Completar Tarea**:
   - Pide el ID de la tarea a completar.
   - Marca la tarea como completada si el ID existe.
   - Muestra un mensaje de error si el ID no existe: "Tarea con ID X no encontrada."
   - Muestra: "Tarea completada."

6. **Eliminar Tarea**:
   - Pide el ID de la tarea a eliminar.
   - Elimina la tarea si el ID existe.
   - Muestra un mensaje de error si el ID no existe: "Tarea con ID X no encontrada."
   - Muestra: "Tarea eliminada."

7. **Persistencia de Datos**:
   - Al salir, guarda la lista en `tasks.json` en formato JSON.
   - Maneja errores al guardar y muestra: "Error al guardar: [detalle]."
   - Carga correctamente las tareas desde `tasks.json` al iniciar.

8. **Manejo de Errores**:
   - Maneja entradas inválidas en el menú (texto en lugar de números).
   - Maneja prioridades inválidas al agregar tareas.
   - Maneja IDs inválidos al completar/eliminar tareas.
   - Maneja errores al leer/escribir el archivo JSON.

9. **Robustez**:
   - El programa no se cierra por entradas incorrectas.
   - Usa `Result` y `Option` para manejar errores de forma segura.
   - Usa referencias mutables (`&mut`) correctamente.

## 🌟 Conceptos de Rust Practicados

Este proyecto te ayudará a aprender:
- **Ownership y Borrowing**: Gestión de la lista de tareas con referencias mutables.
- **Enums y Pattern Matching**: Uso de `Priority` y `match` para manejar comandos.
- **Manejo de Errores**: Uso de `Result` y `Option` para entradas y archivos.
- **Serialización**: Guardar/cargar datos con `serde` y `serde_json`.
- **Entrada/Salida**: Interacción con la consola (`std::io`) y archivos (`std::fs`).

## 📈 Ideas para Extender el Proyecto

Si quieres seguir aprendiendo, prueba a añadir:
- Filtros por prioridad o estado (usa closures y `filter`).
- Edición de tareas existentes (modificar descripción o prioridad).
- Fechas de vencimiento (con la crate `chrono`).
- Interfaz de consola avanzada con la crate `clap`.
- Hilos (`std::thread`) para recordatorios en segundo plano.

## 📚 Recursos
- [Documentación oficial de Rust](https://doc.rust-lang.org)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Community](https://users.rust-lang.org) para dudas y soporte.

## 🤝 Contribuciones
¡Este es un proyecto de aprendizaje! Si tienes sugerencias o mejoras, siéntete libre de abrir un *issue* o un *pull request* en GitHub.

## 📜 Licencia
Este proyecto está bajo la licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.

---

¡Disfruta aprendiendo Rust con este proyecto! 🦀
