use core::pin::pin;
#[cfg(feature = "async")]
use yaag::async_gn_type;
use yaag::{gn, gn_type};

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

#[cfg(feature = "async")]
fn from_fn_async<T, F: FnMut() -> Option<T>>(mut f: F) -> async_gn_type!(T) {
    use std::time::Duration;

    gn!(async move gen {
        while let Some(item) = f() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            yield item;
        }
    } -> T)
}

#[cfg(feature = "async")]
#[tokio::test]
async fn from_fn_works_async() {
    use futures_util::StreamExt;

    let mut c = 0;
    let gn = from_fn_async(|| {
        if c < 10 {
            c += 1;
            Some(c)
        } else {
            None
        }
    });
    assert_eq!(gn.collect::<Vec<_>>().await, (1..=10).collect::<Vec<_>>());
}
