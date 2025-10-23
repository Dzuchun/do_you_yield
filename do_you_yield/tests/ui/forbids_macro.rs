use do_you_yield::gn;

fn main() {
    gn! {
        gen {
            let int = 42;
            let _int = core::pin::pin!(int);
        } -> i32
    };
}
