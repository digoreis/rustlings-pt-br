fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Letra!");
    } else if my_first_initial.is_numeric() {
        println!("Numérico!");
    } else {
        println!("Nem alfabético nem numérico!");
    }

    // Exemplo com um emoji.
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("Letra!");
    } else if your_character.is_numeric() {
        println!("Numérico!");
    } else {
        println!("Nem alfabético nem numérico!");
    }
}
