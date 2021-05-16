use bounded_types::*;

fn main() {
    // this should only fail if type Int is unsigned.
    let _ok: BoundedInt<-5,10> = 3.into();
}