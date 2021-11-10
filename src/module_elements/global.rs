#[derive(Debug,Eq,PartialEq)]
pub struct Global {

}

#[cfg(test)]
mod global_tests {
    use crate::module_elements::global::Global;

    #[test]
    fn test_create() {
        assert_eq!(Global{}, Global{})
    }
}