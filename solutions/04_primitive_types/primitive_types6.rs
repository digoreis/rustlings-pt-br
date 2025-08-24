fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Sintaxe de indexação de tupla.
        let second = numbers.1;

        assert_eq!(second, 2, "Este não é o 2º número na tupla!");
    }
}
