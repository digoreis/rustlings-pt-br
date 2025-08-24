fn call_me(num: u8) {
    for i in 0..num {
        println!("Chamando! Chamando número {}", i + 1);
    }
}

fn main() {
    // `call_me` espera um argumento.
    call_me(5);
}
