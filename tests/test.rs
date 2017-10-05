#![feature(proc_macro)]

extern crate macro_testcase;

use macro_testcase::fold_mac;

struct Foo {
    bar: Box<u32>
}

#[test]
fn test_it_compiles() {
    let foo = Foo {
        bar: Box::new(52)
    };

    /// this won't compile:
    /// error: failed to resolve. Could not find `std` in `{{root}}`
    ///
    /// I suspect this is because of macro hygiene, Idents are all messed up.
    /// It seems to need `std` to find the Deref impl.
    fold_mac! {
        println!("{:?}", foo.bar)
    }
}