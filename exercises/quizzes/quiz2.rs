// Este é um quiz para as seguintes seções:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Vamos construir uma pequena máquina in the form of a function. As input, we're going
// to give a list of strings and commands. Esses comandos determinam que ação
// será aplicada à string. Pode ser:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// A forma exata disso será:
// - A entrada será a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - O elemento de saída será a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
