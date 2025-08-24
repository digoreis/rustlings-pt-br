fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        // Qualquer identificador não utilizado.
        4
    };

    // Em vez de um identificador assim, você usaria um enum em Rust.
    // Mas ainda não chegamos em enums.
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
