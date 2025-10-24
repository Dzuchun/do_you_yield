use core::{
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Yield<O>(Option<O>);

impl<O> Yield<O> {
    /// SAFETY: **never** use this function.
    #[doc(hidden)]
    pub unsafe fn ___make(o: O) -> Self {
        Self(Some(o))
    }
}

impl<O> Future for Yield<O> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        if let Some(data) = unsafe { self.get_unchecked_mut().0.take() } {
            let out = unsafe { &mut *cx.waker().data().cast::<MaybeUninit<O>>().cast_mut() };
            out.write(data);
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
