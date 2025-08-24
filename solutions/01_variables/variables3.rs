#![allow(clippy::needless_late_init)]

fn main() {
    // Ler variáveis não inicializadas não é permitido em Rust!
    // Portanto, precisamos atribuir um valor primeiro.
    let x: i32 = 42;

    println!("Número {x}");

    // É possível declarar uma variável e inicializá-la depois.
    // Mas ela não pode ser usada antes da inicialização.
    let y: i32;
    y = 42;
    println!("Número {y}");
}
