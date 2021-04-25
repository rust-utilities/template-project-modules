pub mod some_module {
    /// Private function may be called within same scope
    fn private_function() {
        println!("Called `some_module::private_function()`");
    }

    /// Public function may be called from other processes
    pub fn public_function() {
        println!("Called `some_module::public_function()`");
    }

    /// Access private_function() with a public function
    pub fn inderect_access() {
        println!("Called `some_module::inderect_access()`");
        private_function();
    }
}

