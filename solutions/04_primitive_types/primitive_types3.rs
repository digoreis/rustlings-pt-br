fn main() {
    // Um array com 100 elementos, todos com o valor 42.
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Uau, isso é um array grande!");
    } else {
        println!("Meh, eu como arrays assim no café da manhã.");
        panic!("Array não é grande o suficiente, precisa de mais elementos");
    }
}
