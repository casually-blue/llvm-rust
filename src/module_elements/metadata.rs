#[derive(Debug, Eq, PartialEq)]
pub struct Metadata {

}

#[cfg(test)]
pub mod metadata_tests {
    use crate::module_elements::metadata::Metadata;

    #[test]
    fn test_create() {
        assert_eq!(Metadata{}, Metadata{})
    }
}