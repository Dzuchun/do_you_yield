use core::{marker::PhantomData, pin::Pin, task::Context};

use crate::not_sync::State;

pub struct Await<F, O>(F, PhantomData<O>);

impl<F, O> Await<F, O> {
    /// SAFETY: **never** use this function.
    #[doc(hidden)]
    pub unsafe fn ___make(f: F) -> Self {
        Self(f, PhantomData)
    }
}

impl<F: Future, O> Future for Await<F, O> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> core::task::Poll<Self::Output> {
        let inner = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };
        let state = unsafe { &mut *cx.waker().data().cast::<State<O>>().cast_mut() };
        let waker = state.waker.clone();
        let mut cx_inner = Context::from_waker(&waker);
        inner.poll(&mut cx_inner)
    }
}
