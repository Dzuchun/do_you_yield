use std::pin::pin;

use do_you_yield::gn;

#[test]
fn empty() {
    let gn = gn!(gen{} -> i32);
    let gn = pin!(gn);
    assert_eq!(gn.collect::<Vec<_>>(), Vec::<i32>::new());
}

#[cfg(feature = "async")]
#[tokio::test]
async fn empty_async() {
    use futures_util::StreamExt;
    use std::time::Duration;

    let gn = gn!(async gen { tokio::time::sleep(Duration::from_secs(1)).await; } -> i32);
    assert_eq!(gn.collect::<Vec<_>>().await, vec![]);
}

#[test]
fn single() {
    let gn = gn!(gen{ yield 42; } -> i32);
    let gn = pin!(gn);
    assert_eq!(gn.collect::<Vec<_>>(), vec![42]);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn single_async() {
    use futures_util::StreamExt;
    use std::time::Duration;

    let gn = gn!(async gen {
        tokio::time::sleep(Duration::from_secs(1)).await;
        yield 42;
        tokio::time::sleep(Duration::from_secs(1)).await;
    } -> i32);
    assert_eq!(gn.collect::<Vec<_>>().await, vec![42]);
}

#[test]
fn generates_1_10() {
    let gn = gn!(gen {
        let start = 1;
        let end = 10;
        for i in start..=end {
            yield i;
        }
    } -> i32);
    let gn = pin!(gn);
    let it = 1..=10;
    assert_eq!(it.collect::<Vec<_>>(), gn.collect::<Vec<_>>());
}

#[cfg(feature = "async")]
#[tokio::test]
async fn generates_1_10_async() {
    use std::time::Duration;

    use futures_util::StreamExt;

    let gn = gn!(async gen {
        let start = 1;
        let end = 10;
        for i in start..=end {
            tokio::time::sleep(Duration::from_millis(100)).await;
            yield i;
        }
    } -> i32);
    let it = 1..=10;
    assert_eq!(it.collect::<Vec<_>>(), gn.collect::<Vec<_>>().await);
}

#[test]
fn captures_ref() {
    let mut i = 1;
    let gn = do_you_yield::gn!(gen {
        while i <= 10 {
            yield i;
            i += 1;
        }
    } -> i32);
    let gn = core::pin::pin!(gn);
    let it = 1..=10;
    assert_eq!(it.collect::<Vec<_>>(), gn.collect::<Vec<_>>());
}

#[cfg(feature = "async")]
#[tokio::test]
async fn captures_ref_async() {
    use futures_util::StreamExt;

    let mut i = 1;
    let wait = async |val: i32| {
        use std::time::Duration;
        tokio::time::sleep(Duration::from_millis(100)).await;
        val
    };
    let gn = do_you_yield::gn!(async gen {
        while i <= 10 {
            yield wait(i).await;
            i += 1;
        }
    } -> i32);
    let it = 1..=10;
    assert_eq!(it.collect::<Vec<_>>(), gn.collect::<Vec<_>>().await);
}
