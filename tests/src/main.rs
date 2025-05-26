#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        println!("asserting");
        assert_eq!(main(), ());
    }

    #[test]
    #[should_panic(expected = "wtf")]
    fn sample2() {
        panic!("wtf");
    }

    #[test]
    //#[should_panic]
    fn sample3() -> Result<(), String> {
        if !true {
            Err(String::from("error"))
        } else {
            assert!(errora().is_err());
            Ok(())
        }
    }
}

fn errora() -> Result<(), String> {
    Err(String::from("errora"))
}

fn main() {
    println!("Hello, world!");
}
