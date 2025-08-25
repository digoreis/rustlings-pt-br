#![allow(clippy::ptr_arg)]

// TODO: Corrija os erros do compilador sem alterar nada exceto adicionar ou
// remover referências (o caractere `&`).

// Não deve tomar posse
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Deve tomar posse
fn string_uppercase(mut data: &String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}
