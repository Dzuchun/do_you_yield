use core::pin::pin;
use do_you_yield::{gn, gn_type};

fn once<T>(t: T) -> gn_type!(T) {
    gn!(gen { yield t; } -> T)
}

#[test]
fn once_works_i32() {
    let once = once(42);
    let once = pin!(once);
    assert_eq!(once.collect::<Vec<_>>(), vec![42]);
}

#[test]
fn once_works_box() {
    let once = once(Box::new(42));
    let once = pin!(once);
    assert_eq!(once.collect::<Vec<_>>(), vec![Box::new(42)]);
}
