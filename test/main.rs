#[test]
fn main() {
    let case = trybuild::TestCases::new();
    case.pass("test/case/impls.rs");
}