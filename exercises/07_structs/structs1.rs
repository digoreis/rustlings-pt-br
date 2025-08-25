struct ColorRegularStruct {
    // TODO: Adicione os campos que o teste `regular_structs` espera.
    // Que tipos os campos devem ter? Quais são os valores mínimo e máximo para cores RGB?
}

struct ColorTupleStruct(/* TODO: Adicione os campos que o teste `tuple_structs` espera */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instancie uma struct regular.
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instancie uma struct tupla.
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instancie uma struct unit.
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
