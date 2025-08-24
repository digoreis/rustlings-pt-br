fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use um índice de tupla para acessar o segundo elemento de `numbers`
        // e atribua-o a uma variável chamada `second`.
        // let second = ???;

        assert_eq!(second, 2, "Este não é o 2º número na tupla!");
    }
}
