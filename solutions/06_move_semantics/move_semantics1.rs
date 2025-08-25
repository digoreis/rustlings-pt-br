fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ added

    vec.push(88);

    vec
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `vec0` não pode mais ser acessado porque foi movido para `fill_vec`.
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
