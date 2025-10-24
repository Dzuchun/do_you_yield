use crate::waker::make;
use core::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll, Waker},
};

#[macro_export]
macro_rules! async_gn_type {
    ($t:ty) => {
        $crate::not_sync::Gn<impl ::core::future::Future<Output = ()>, $t>
    };
}

mod yld;
use futures_core::Stream;
pub use yld::Yield;

mod awt;
pub use awt::Await;

struct State<O> {
    pub out: Option<O>,
    pub waker: Waker,
}

pub struct Gn<F: Future<Output = ()>, O> {
    pub fut: F,
    pub _ph: PhantomData<O>,
}

impl<F: Future<Output = ()>, O> Gn<F, O> {
    // signature is based on the one proposed for stdlib
    fn gn_poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Option<Poll<O>> {
        let fut;
        unsafe {
            let self_ = self.as_mut().get_unchecked_mut();
            fut = Pin::new_unchecked(&mut self_.fut);
        }
        let mut state = State {
            out: None,
            waker: cx.waker().clone(),
        };
        let waker = make((&raw mut state).cast_const().cast());
        let poll = fut.poll(&mut Context::from_waker(&waker));
        // no more references to `state` exist at this point
        match poll {
            core::task::Poll::Ready(()) => {
                // finished generation, nothing to return
                None
            }
            core::task::Poll::Pending => {
                // item was maybe-saved in the output
                Some(match state.out {
                    Some(item) => Poll::Ready(item),
                    None => Poll::Pending,
                })
            }
        }
    }
}

#[inline]
fn transpose<T>(value: Option<Poll<T>>) -> Poll<Option<T>> {
    match value {
        Some(Poll::Pending) => Poll::Pending,
        Some(Poll::Ready(item)) => Poll::Ready(Some(item)),
        None => Poll::Ready(None),
    }
}

impl<F: Future<Output = ()>, O> Stream for Gn<F, O> {
    type Item = O;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        transpose(self.gn_poll_next(cx))
    }
}
