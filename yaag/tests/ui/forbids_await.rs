use yaag::gn;

fn main() {
    gn! {
        gen {
            core::future::ready::<i32>(42).await;
        } -> i32
    };
}
