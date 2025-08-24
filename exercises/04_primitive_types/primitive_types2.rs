// Characters (`char`)

fn main() {
    // Note que são _aspas simples_, elas são diferentes das aspas duplas
    // que você tem visto por aí.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Letra!");
    } else if my_first_initial.is_numeric() {
        println!("N~umero!");
    } else {
        println!("Nem letra e nem número!");
    }

    // TODO: De forma análoga ao exemplo anterior, declare uma variável chamada `your_character`
    // abaixo com seu caractere favorito.
    // Experimente uma letra, um dígito (entre aspas simples), um caractere especial, um caractere
    // de um idioma diferente do seu, ou até um emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Letra!");
    } else if your_character.is_numeric() {
        println!("Número!");
    } else {
        println!("Nem letra e nem número!");
    }
}
