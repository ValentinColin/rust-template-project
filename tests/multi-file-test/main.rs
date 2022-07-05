use rust_template_project::add;

mod test_module;
use crate::test_module::setup;

#[test]
fn test_add() {
    setup();
    assert_eq!(add(3, 2), 5);
}
