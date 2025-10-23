use std::pin::pin;

use do_you_yield::gn;

#[test]
fn empty() {
    let gn = gn!(gen{} -> i32);
    let gn = pin!(gn);
    assert_eq!(gn.collect::<Vec<_>>(), vec![]);
}

#[test]
fn single() {
    let gn = gn!(gen{ yield 42; } -> i32);
    let gn = pin!(gn);
    assert_eq!(gn.collect::<Vec<_>>(), vec![42]);
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
