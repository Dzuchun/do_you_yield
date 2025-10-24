use core::{
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
};

#[macro_export]
macro_rules! gn_type {
    ($t:ty) => {
        $crate::sync::Gn<impl ::core::future::Future<Output = ()>, $t>
    };
}

mod yld;
#[doc(hidden)]
pub use yld::Yield;

#[doc(hidden)]
pub struct Gn<F: Future<Output = ()>, O> {
    pub fut: F,
    pub out: MaybeUninit<O>,
}

impl<F: Future<Output = ()>, O> Gn<F, O> {
    fn gn_next(mut self: Pin<&mut Self>) -> Option<O> {
        let fut;
        let out;
        unsafe {
            let self_ = self.as_mut().get_unchecked_mut();
            fut = Pin::new_unchecked(&mut self_.fut);
            out = &mut self_.out;
        }
        let waker = make((&raw mut state).cast_const().cast());
        let _ = out;
        match fut.poll(&mut Context::from_waker(&waker)) {
            core::task::Poll::Ready(()) => {
                // finished generation
                None
            }
            core::task::Poll::Pending => {
                // item was saved into out
                Some(unsafe { self.get_unchecked_mut().out.assume_init_read() })
            }
        }
    }
}

impl<F: Future<Output = ()>, O> Iterator for Pin<&mut Gn<F, O>> {
    type Item = O;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.as_mut().gn_next()
    }
}

pub trait Generator
where
    for<'a> Pin<&'a mut Self>: IntoIterator<Item = Self::Item>,
{
    type Item;
}

impl<F: Future<Output = ()>, O> Generator for Gn<F, O> {
    type Item = O;
}
