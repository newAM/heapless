use trybuild::TestCases;

fn main() {
    let t = TestCases::new();
    // forces trybuild to do "cargo build" instead of "cargo check"
    // checking is not enough to trip const assertions
    // https://github.com/dtolnay/trybuild/issues/258
    t.pass("force_build.rs");
    t.compile_fail("ui/*.rs");
}
