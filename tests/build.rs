
#[test]
fn trybuild_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/pass/*.rs");
}

#[test]
fn trybuild_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/build/fail/*.rs");
}