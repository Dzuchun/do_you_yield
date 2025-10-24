use yaag::gn;

fn main() {
    gn! {
        gen {
            let int = 42;
            let _int = core::pin::pin!(int);
        } -> i32
    };
}
