#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]

use {{project-name}};

#[test]
fn it_can_run_without_failing() {
    let result = {{project-name}}::run(7);
    assert!(result.is_ok())
}

// #[test]
// fn it_has_a_nasty_bug() {
//     let result = {{project-name}}::run(0);
//     assert!(result.is_ok())
// }
