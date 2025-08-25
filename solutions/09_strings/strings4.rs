fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // Aqui, ambas as respostas funcionam.
    // `.into()` converte um tipo no tipo esperado.
    // Se for chamado onde `String` é esperado, converterá `&str` para `String`.
    string("nice weather".into());
    // Mas se for chamado onde `&str` é esperado, então `&str` permanece `&str` já que nenhuma conversão é necessária.
    // Se você remover a linha `#[allow(…)]`, então Clippy dirá para remover `.into()` abaixo já que é uma conversão inútil.
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // AVISO: Isso é indexação de bytes, não indexação de caracteres.
    // Indexação de caracteres pode ser feita usando `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
