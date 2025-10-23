#[test]
fn generates_1_10() {
    let gn = do_you_yield::gn!(gen {
        let start = 1;
        let end = 10;
        for i in (start..=end) {
            yield i;
        }
    } -> i32);
    let gn = core::pin::pin!(gn);
    let it = 1..=10;
    assert_eq!(it.collect::<Vec<_>>(), gn.collect::<Vec<_>>());
}
