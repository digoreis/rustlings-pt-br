fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Pegue uma fatia chamada `nice_slice` do array `a` para que o teste passe.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
