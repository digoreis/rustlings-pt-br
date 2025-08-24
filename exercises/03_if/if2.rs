// TODO: Corrija o erro de compilação nesta função.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" { "Yummy!" } else { 1 }
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

// TODO: Leia os testes para entender o comportamento desejado.
// Faça todos os testes passarem sem alterá-los.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // Isso significa que chamar `picky_eater` com o argumento "strawberry" deve retornar "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
