// O tipo dos argumentos da função deve ser anotado.
// Adicionada a anotação de tipo `u64`.
fn call_me(num: u64) {
    for i in 0..num {
        println!("Chamando! Número chamando {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
