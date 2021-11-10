use crate::module_elements::{
    global::*,
    metadata::*,
    function::*
};

#[derive(Debug, Eq, PartialEq)]
pub struct Module {
    name: String,
    functions: Vec<Function>,
    globals: Vec<GlobalVariable>,
    metadata: Vec<Metadata>
}

impl Module {
    pub fn new(name: String) -> Self {
        Module { name, functions: vec![], globals: vec![], metadata: vec![] }
    }
}

#[cfg(test)]
mod module_tests {
    use crate::module_elements::module::Module;
    use crate::str;

    #[test]
    fn create_module() {
        assert_eq!(
            Module::new(str!("main")),
            Module {name: str!("main"), functions: vec![], globals: vec![], metadata: vec![] })
    }
}