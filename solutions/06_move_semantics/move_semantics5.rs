#![allow(clippy::ptr_arg)]

// Empresta em vez de tomar posse.
// É recomendado usar `&str` em vez de `&String` aqui. Mas isso é
// suficiente por agora porque ainda não lidamos com strings.
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Toma posse em vez de emprestar.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
