
#[test]
fn trybuild_pass_named() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/pass/01.*.rs");
}

#[test]
fn trybuild_pass_unnamed() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/pass/02.*.rs");
}

#[test]
fn trybuild_pass_named_generics() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/pass/03.*.rs");
}

#[test]
fn trybuild_pass_unnamed_generics() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/pass/04.*.rs");
}

#[test]
fn trybuild_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/build/fail/*.rs");
}