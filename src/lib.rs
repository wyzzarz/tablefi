pub fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Hello, world!");
    Ok(())
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        assert!(hello_world().is_ok());
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
