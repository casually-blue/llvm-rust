use crate::module_elements::Global;

#[derive(Debug,Eq,PartialEq)]
pub struct GlobalVariable {

}

impl Global for GlobalVariable {

}

#[cfg(test)]
mod global_tests {
    use crate::module_elements::global::GlobalVariable;

    #[test]
    fn test_create() {
        assert_eq!(GlobalVariable{}, GlobalVariable{})
    }
}