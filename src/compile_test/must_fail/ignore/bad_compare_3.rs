use bounded_types::*;

fn main() {
    let ok: BoundedInt<0,10> = 1.into();
    let unbounded: Unbounded<_,_> = ok.into_unbounded();
    assert!(unbounded == 1);
}