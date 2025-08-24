fn main() {
    let number = "T-H-R-E-E";
    println!("Soletrando um número: {number}");

    // Usando sombreamento de variável
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("Número mais dois é: {}", number + 2);
}
