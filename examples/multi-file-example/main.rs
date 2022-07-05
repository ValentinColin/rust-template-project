use rust_template_project::lib_hello;

mod ex_module;
use crate::ex_module::setup;

fn main() {
    setup();
    lib_hello();
}
