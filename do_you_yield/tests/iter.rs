use core::pin::pin;
use do_you_yield::{gn, gn_type};

fn from_fn<T, F: FnMut() -> Option<T>>(mut f: F) -> gn_type!(T) {
    gn!(move gen {
        while let Some(item) = f() {
            yield item;
        }
    } -> T)
}

#[test]
fn from_fn_works() {
    let mut c = 0;
    let gn = from_fn(|| {
        if c < 10 {
            c += 1;
            Some(c)
        } else {
            None
        }
    });
    let gn = pin!(gn);
    assert_eq!(gn.collect::<Vec<_>>(), (1..=10).collect::<Vec<_>>());
}
