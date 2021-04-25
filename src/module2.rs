/// Note, prior to ~2015 this sort of module supposedly was not an option
pub mod another_module {
    /// Private function may be called within same scope
    fn private_function() {
        println!("Called `some_module::private_function()`");
    }

    /// Public function may be called from other processes
    pub fn public_function() {
        println!("Called `some_module::public_function()`");
    }

    /// Access private_funciton() with a public function
    pub fn inderect_access() {
        println!("Called `some_module::inderect_access()`");
        private_function();
    }
}
