fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green");

    if is_a_color_word(&word) {
        //             ^ adicionado para ter `&String` que é automaticamente
        //               convertido para `&str` pelo compilador.
        println!("Essa é uma palavra de cor que eu conheço!");
    } else {
        println!("Essa não é uma palavra de cor que eu conheço.");
    }
}
