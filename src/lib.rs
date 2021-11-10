pub mod module_elements;
pub mod attributes;

#[macro_export]
macro_rules! str {
    ($str:tt) => {$str.to_string()};
}
