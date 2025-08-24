fn square(num: i32) -> i32 {
    // Removido o ponto e vírgula `;` ao final da linha abaixo para retornar implicitamente o resultado.
    num * num
}

fn main() {
    let answer = square(3);
    println!("O quadrado de 3 é {answer}");
}
