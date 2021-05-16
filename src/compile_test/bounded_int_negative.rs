use bounded_types::*;

fn main() {
    let val1_ok: BoundedI64<-10, 0> = (-10).into();
    let val2_ok: BoundedI64<-10, 0> = 0.into();
    let val3_ok: BoundedI64<-10, 30> = (-5).into();
    let val4_ok: BoundedI64<-10, 30> = 5.into();

    let val1_err: BoundedI64<-10, 0> = (-11).into();
    let val2_err: BoundedI64<-10, 0> = 1.into();
    let val3_err: BoundedI64<0, -10> = 5.into();

    assert!(val1_ok.is_ok());
    assert!(val2_ok.is_ok());
    assert!(val3_ok.is_ok());
    assert!(val4_ok.is_ok());

    assert!(val1_err.is_err());
    assert!(val2_err.is_err());
    assert!(val3_err.is_err());
}