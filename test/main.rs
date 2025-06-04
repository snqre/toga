#[test]
fn main() {
    let t = trybuild::TestCases::new();
    t.pass("test/case/simple_model.rs");
    t.pass("test/case/injectable.rs");
}