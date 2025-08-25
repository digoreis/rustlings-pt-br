// TODO: Corrija o erro do compilador sobre chamar uma função privada.
mod sausage_factory {
    // Não deixe ninguém de fora deste módulo ver isso!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
