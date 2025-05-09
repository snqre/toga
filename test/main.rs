#[test]
fn main() {
    let t = trybuild::TestCases::new();
    t.pass("test/case/simple_model.rs");
}