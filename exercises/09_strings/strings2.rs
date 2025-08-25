// TODO: Corrija o erro do compilador na função `main` sem alterar esta função.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Não altere essa linha.

    if is_a_color_word(word) {
        println!("Essa é uma palavra de cor que eu conheço!");
    } else {
        println!("Essa não é uma palavra de cor que eu conheço.");
    }
}
