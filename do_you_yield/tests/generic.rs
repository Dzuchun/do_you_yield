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

#[cfg(feature = "async")]
fn once_async<T>(t: T) -> do_you_yield::async_gn_type!(T) {
    use std::time::Duration;

    gn!(async gen {
        tokio::time::sleep(Duration::from_millis(100)).await;
        yield t;
    } -> T)
}

#[cfg(feature = "async")]
#[tokio::test]
async fn once_async_works_i32() {
    use futures_util::StreamExt;

    let once = once_async(42);
    assert_eq!(once.collect::<Vec<_>>().await, vec![42]);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn once_async_works_box() {
    use futures_util::StreamExt;

    let once = once_async(Box::new(42));
    assert_eq!(once.collect::<Vec<_>>().await, vec![Box::new(42)]);
}
