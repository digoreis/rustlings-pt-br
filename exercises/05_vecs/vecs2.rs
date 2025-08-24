fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiplique cada elemento do slice `input` por 2 e adicione ao
        // vetor `output`.
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Um exemplo de como coletar um vetor após o mapeamento.
    // Mapeamos cada elemento do slice `input` para seu valor mais 1.
    // Se a entrada for `[1, 2, 3]`, a saída será `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Aqui, também queremos multiplicar cada elemento do slice `input`
    // por 2, mas usando o mapeamento do iterador ao invés de adicionar manualmente em um vetor vazio.
    // Veja o exemplo na função `vec_map_example` acima.
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
