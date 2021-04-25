/// Note, `crate` keyword allows access to modules from `src/` directory
pub mod sticky_module {
    use crate::module1::some_module;
    use crate::module2::another_module;

    pub fn some_module_public_function() {
        println!("Called `sticky_module::some_module_public_function()`");
        some_module::public_function();
    }

    pub fn another_module_public_function() {
        println!("Called `sticky_module::another_module_public_function()`");
        another_module::public_function();
    }
}
