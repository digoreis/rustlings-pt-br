fn current_favorite_color() -> String {
    // Equivalente a `String::from("blue")`
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("Minha cor favorita atual é {answer}");
}
