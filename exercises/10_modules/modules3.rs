// Você pode usar a palavra-chave `use` para trazer caminhos de módulos de módulos de
// qualquer lugar e especialmente da biblioteca padrão para o seu escopo.

// TODO: Traga `SystemTime` e `UNIX_EPOCH` do módulo `std::time` para
// o seu escopo. Pontos de estilo extras se você conseguir fazer isso com uma linha!
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
