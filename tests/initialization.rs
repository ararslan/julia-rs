extern crate julia;

use julia::Julia;

// TODO: This is absolute garbage.
#[test]
fn init_works() {
    let jl = Julia::new("/Applications/Julia-0.5.app/Contents/Resources/julia/lib");
    assert!(true)
}
