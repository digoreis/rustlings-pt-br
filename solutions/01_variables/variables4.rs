fn main() {
    // Em Rust, variáveis são imutáveis por padrão.
    // Adicionar a palavra-chave `mut` após o `let` torna a variável declarada mutável.
    let mut x = 3;
    println!("Número {x}");

    x = 5;
    println!("Número {x}");
}
