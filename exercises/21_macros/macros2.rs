fn main() {
    my_macro!();
}

// TODO: Corrija o erro do compilador by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
