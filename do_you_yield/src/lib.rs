#![cfg_attr(not(test), no_std)]

use core::{
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

#[doc(hidden)]
pub struct Gn<F: Future<Output = ()>, O> {
    pub fut: F,
    pub out: MaybeUninit<O>,
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
unsafe fn clone(data: *const ()) -> RawWaker {
    RawWaker::new(data, &VTABLE)
}
unsafe fn wake(_: *const ()) {}
unsafe fn wake_by_ref(_: *const ()) {}
unsafe fn drop(_: *const ()) {}

impl<F: Future<Output = ()>, O> Gn<F, O> {
    fn gn_next(mut self: Pin<&mut Self>) -> Option<O> {
        let fut;
        let out;
        unsafe {
            let self_ = self.as_mut().get_unchecked_mut();
            fut = Pin::new_unchecked(&mut self_.fut);
            out = &mut self_.out;
        }
        let waker = unsafe { Waker::new(core::ptr::from_mut(out).cast_const().cast(), &VTABLE) };
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

#[doc(hidden)]
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

#[macro_export]
macro_rules! gn {
    (gen {$($code:tt)*} -> $out:ty) => {
        $crate::Gn::<_, $out> {
            fut: async {
                $crate::gn!(@trans $out | $($code)*);
            },
            out: ::core::mem::MaybeUninit::uninit(),
        }
    };
    (@trans $out:ty | $t:tt.await; $($rest:tt)*) => { ::core::compile_error!("Awaiting in the generator is not supported"); };
    (@trans $out:ty |
        for $i:ident in $it:tt {
            $($code:tt)*
        }
        $($rest:tt)*
    ) => {
        #[allow(unused_parens)]
        let iter = $crate::gn!(@trans $out | $it);
        for $i in iter {
            $crate::gn!(@trans $out | $($code)*);
        }
        $crate::gn!(@trans $out | $($rest)*);
    };
    (@trans $out:ty | let $i:ident; $($rest:tt)*) => {
        let $i;
        $crate::gn!(@trans $out | $($rest)*)
    };
    (@trans $out:ty | let $p:pat = $e:tt; $($rest:tt)*) => {
        let $p = $crate::gn!(@trans $out | $e);
        $crate::gn!(@trans $out | $($rest)*)
    };
    (@trans $out:ty | yield $y:tt; $($rest:tt)*) => {
        unsafe { $crate::Yield::<$out>::___make($crate::gn!(@trans $out | $y)) }.await;
        $crate::gn!(@trans $out | $($rest)*)
    };
    // nil rule
    (@trans $out:ty |) => {};
    // all other kind of expressions are passed unchanged
    (@trans $out:ty | $e:expr) => { $e };
    // item bypass (without tail drop)
    (@trans $out:ty | $it:item $($rest:tt)*) => {
        $it $crate::gn!(@trans $out | $($rest)*)
    };
    // bypass rule, just in case I forgor something
    (@trans $out:ty | $($bp:tt)+) => {$($bp)+};
}
