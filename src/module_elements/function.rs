#[derive(Debug,Eq, PartialEq)]
pub struct Function {

}

#[cfg(test)]
pub mod function_tests {
    use crate::module_elements::function::Function;

    #[test]
    fn test_creation() {
        assert_eq!(Function {}, Function {})
    }
}