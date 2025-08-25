// Você pode trazer caminhos de módulos para escopos e fornecer novos nomes para eles com
// as palavras-chave `use` e `as`.

mod delicious_snacks {
    // TODO: Adicione as duas declarações `use` a seguir depois de corrigi-las.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
