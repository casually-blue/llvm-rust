mod module_elements;

#[macro_export]
macro_rules! str {
    ($str:tt) => {$str.to_string()};
}