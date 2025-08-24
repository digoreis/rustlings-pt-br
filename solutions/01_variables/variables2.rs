fn main() {
    // A maneira mais fácil de corrigir o erro de compilação é inicializar a
    // variável `x`. Ao definir seu valor como um inteiro, o Rust infere seu tipo
    // como `i32`, que é o tipo padrão para inteiros.
    let x = 42;

    // Mas podemos forçar um tipo diferente do padrão `i32` adicionando
    // uma anotação de tipo:
    // let x: u8 = 42;

    if x == 10 {
        println!("x é dez!");
    } else {
        println!("x não é dez!");
    }
}
