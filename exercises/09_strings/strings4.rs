// Chamadas desta função devem ser substituídas por chamadas de `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Aqui temos vários valores - alguns são `String`, alguns são `&str`.
// Sua tarefa é substituir `placeholder(…)` por `string_slice(…)`
// ou `string(…)` dependendo do que você acha que cada valor é.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // AVISO: Isso é indexação de bytes, não indexação de caracteres.
    // Indexação de caracteres pode ser feita usando `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
