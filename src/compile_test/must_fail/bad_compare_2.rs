use bounded_types::*;

fn main() {
    let ok: BoundedI64<0,10> = 1.into();
    assert!(ok);
}