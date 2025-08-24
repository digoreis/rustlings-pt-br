fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- índices
        //          -------
        //             |
        //             +--- slice

        // Note que o índice superior 4 é excluído.
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // O índice superior pode ser incluído usando a sintaxe `..=` (com sinal de `=`)
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
