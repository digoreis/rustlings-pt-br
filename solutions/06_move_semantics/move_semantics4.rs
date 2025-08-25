fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // `y` usado aqui.
        y.push(42);
        // A referência mutável `y` não é mais usada,
        // portanto uma nova referência pode ser criada.
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
