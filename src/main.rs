/// Public module named `some_module` from `src/module1/mod.rs` file
mod module1;
use module1::some_module;

/// Public module named `another_module` from `src/module2.rs` file
mod module2;
use module2::another_module;

/// Public module named `sticky_module` from `src/glue/mod.rs` file
mod glue;
use glue::sticky_module;


/// This is first function called when the executable is run
fn main() {
    some_module::public_function();
    some_module::inderect_access();

    println!("#--------------------------------------#");

    another_module::public_function();
    another_module::inderect_access();

    println!("#--------------------------------------#");

    sticky_module::some_module_public_function();
    sticky_module::another_module_public_function();
}

