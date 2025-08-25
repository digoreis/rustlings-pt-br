// Vamos construir uma pequena máquina in the form of a function. As input, we're going
// to give a list of strings and commands. Esses comandos determinam que ação
// será aplicada à string. Pode ser:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// A forma exata disso será:
// - A entrada será a vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - O elemento de saída será a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // The solution with a loop. Check out `transformer_iter` for a version
    // with iterators.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            // Create the new string.
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            };

            // Push the new string to the output vector.
            output.push(new_string);
        }

        output
    }

    // Equivalent to `transform` but uses an iterator instead of a loop for
    // comparison. Don't worry, we will practice iterators later ;)
    pub fn transformer_iter(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    // Import `transformer`.
    use super::my_module::transformer;

    use super::Command;
    use super::my_module::transformer_iter;

    #[test]
    fn it_works() {
        for transformer in [transformer, transformer_iter] {
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
}
