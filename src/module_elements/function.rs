use crate::module_elements::Global;

#[derive(Debug,Eq, PartialEq)]
pub struct Function {

}

impl Global for Function {}

#[cfg(test)]
pub mod function_tests {
    use crate::module_elements::function::Function;

    #[test]
    fn test_creation() {
        assert_eq!(Function {}, Function {})
    }
}