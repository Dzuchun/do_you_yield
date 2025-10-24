use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::not_sync::State;

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
            let state = unsafe { &mut *cx.waker().data().cast::<State<O>>().cast_mut() };
            state.out = Some(data);
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
