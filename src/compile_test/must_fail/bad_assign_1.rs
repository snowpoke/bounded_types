use bounded_types::*;

fn main() {
    let _ok: BoundedI64<0,10> = (std::f32::consts::PI).into();
}