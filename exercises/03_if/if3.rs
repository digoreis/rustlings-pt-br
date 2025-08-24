fn animal_habitat(animal: &str) -> &str {
    // TODO: Arrume o erro de compilação na instrução abaixo.
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3
    } else {
        "Unknown"
    };

    // Não altere a expressão abaixo!
    if identifier == 1 {
        "Praia"
    } else if identifier == 2 {
        "Toca"
    } else if identifier == 3 {
        "Deserto"
    } else {
        "Desconhecido"
    }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

// Não altere os testes!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Toca")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Deserto")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Praia")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Desconhecido")
    }
}
