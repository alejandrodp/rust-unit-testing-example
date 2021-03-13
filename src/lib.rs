pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

// Esta función falla al propio, con el fin de mostrar lo que pasa
// cuando una prueba falla. 
fn suma_incorrectamente(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // se importan las funciones
    use super::*;

    #[test]
    fn probar_suma() {
            assert_eq!(suma(1, 2), 3);
    }

    #[test]
    fn probar_suma_incorrectamente() {
            // Claramente esta aserción va a causar un panic
            assert_eq!(suma_incorrectamente(1, 2), 3);
    }
}
