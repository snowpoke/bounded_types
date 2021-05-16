use bounded_types::*;

fn main() {
    let ok: BoundedInt<0,10> = 1.into();
    assert!(ok);
}